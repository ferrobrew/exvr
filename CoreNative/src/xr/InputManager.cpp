#include "xr/InputManager.h"
#include "xr/Session.h"
#include "xr/XRUtils.h"
#include "xr/XR.h"
#include "xr/SessionManager.h"

#include "Log.h"

#include <array>
#include <string>

namespace xr
{
constexpr XrQuaternionf ACTION_ORIENTATION{ 0, 0, 0, 1 };

Action::Action(XrSession session, XrActionSet actionSet, XrActionType type, XrPath const* subactionPaths,
			   size_t subactionCount, std::string const& actionName, std::string const& localizedName)
{
	XrActionCreateInfo actionCreateInfo = { XR_TYPE_ACTION_CREATE_INFO };
	actionCreateInfo.actionType = type;
	strcpy_s(actionCreateInfo.actionName, actionName.c_str());
	strcpy_s(actionCreateInfo.localizedActionName, localizedName.c_str());
	actionCreateInfo.countSubactionPaths = uint32_t(subactionCount);
	actionCreateInfo.subactionPaths = subactionPaths;
	CHECK_XRCMD(xrCreateAction(actionSet, &actionCreateInfo, &m_Action));

	if (type == XR_ACTION_TYPE_POSE_INPUT)
	{
		XrActionSpaceCreateInfo spaceCreateInfo = { XR_TYPE_ACTION_SPACE_CREATE_INFO };
		spaceCreateInfo.poseInActionSpace.orientation = ACTION_ORIENTATION;
		spaceCreateInfo.action = m_Action;
		// spaceCreateInfo.subactionPath = hmm;
		CHECK_XRCMD(xrCreateActionSpace(session, &spaceCreateInfo, &m_Space));
	}

	m_ActionType = type;
	m_LocalizedName = localizedName;
}

void Action::LogBinding(const Session& session) const
{
	XrBoundSourcesForActionEnumerateInfo getInfo = { XR_TYPE_BOUND_SOURCES_FOR_ACTION_ENUMERATE_INFO };
	getInfo.action = m_Action;
	uint32_t pathCount = 0;
	CHECK_XRCMD(xrEnumerateBoundSourcesForAction(session, &getInfo, 0, &pathCount, nullptr));
	std::vector<XrPath> paths(pathCount);
	CHECK_XRCMD(xrEnumerateBoundSourcesForAction(session, &getInfo, uint32_t(paths.size()), &pathCount, paths.data()));

	std::string sourceName;
	for (uint32_t i = 0; i < pathCount; ++i)
	{
		constexpr XrInputSourceLocalizedNameFlags all = XR_INPUT_SOURCE_LOCALIZED_NAME_USER_PATH_BIT |
														XR_INPUT_SOURCE_LOCALIZED_NAME_INTERACTION_PROFILE_BIT |
														XR_INPUT_SOURCE_LOCALIZED_NAME_COMPONENT_BIT;

		XrInputSourceLocalizedNameGetInfo nameInfo = { XR_TYPE_INPUT_SOURCE_LOCALIZED_NAME_GET_INFO };
		nameInfo.sourcePath = paths[i];
		nameInfo.whichComponents = all;

		uint32_t size = 0;
		CHECK_XRCMD(xrGetInputSourceLocalizedName(session, &nameInfo, 0, &size, nullptr));
		if (size < 1)
		{
			continue;
		}
		std::vector<char> grabSource(size);
		CHECK_XRCMD(
			xrGetInputSourceLocalizedName(session, &nameInfo, uint32_t(grabSource.size()), &size, grabSource.data()));
		if (!sourceName.empty())
		{
			sourceName += " and ";
		}
		sourceName += "'";
		sourceName += std::string(grabSource.data(), size - 1);
		sourceName += "'";
	}

	Logf("%s action is bound to %s\n", m_LocalizedName.c_str(),
		 ((!sourceName.empty()) ? sourceName.c_str() : "nothing"));
}

bool Action::Update(const Session& session)
{
	bool active = false;
	XrActionStateGetInfo getInfo{ XR_TYPE_ACTION_STATE_GET_INFO };
	getInfo.action = m_Action;

	switch (m_ActionType)
	{
	case XR_ACTION_TYPE_BOOLEAN_INPUT: {
		XrActionStateBoolean state{ XR_TYPE_ACTION_STATE_BOOLEAN };
		active = XR_SUCCEEDED(xrGetActionStateBoolean(session, &getInfo, &state)) && state.isActive;
		m_Boolean.m_WasPressed = m_Boolean.m_IsPressed;
		m_Boolean.m_IsPressed = state.currentState;
		break;
	}
	case XR_ACTION_TYPE_FLOAT_INPUT: {
		XrActionStateFloat state{ XR_TYPE_ACTION_STATE_FLOAT };
		active = XR_SUCCEEDED(xrGetActionStateFloat(session, &getInfo, &state)) && state.isActive;
		m_Float.m_Value = state.currentState;
		break;
	}
	case XR_ACTION_TYPE_VECTOR2F_INPUT: {
		XrActionStateVector2f state{ XR_TYPE_ACTION_STATE_VECTOR2F };
		active = XR_SUCCEEDED(xrGetActionStateVector2f(session, &getInfo, &state)) && state.isActive;
		m_Vector.m_Value = state.currentState;
		break;
	}
	case XR_ACTION_TYPE_POSE_INPUT: {
		XrActionStatePose state{ XR_TYPE_ACTION_STATE_POSE };
		active = XR_SUCCEEDED(xrGetActionStatePose(session, &getInfo, &state)) && state.isActive;
		session.GetPose(m_Pose.m_Value, m_Space);
		break;
	}
	}

	return active;
}

bool SuggestActions(XrInstance instance, const char* profile, XrAction* actions, XrPath* paths, size_t count)
{
	XrPath path;
	XrResult result = xrStringToPath(instance, profile, &path);

	if (!XR_SUCCEEDED(result))
	{
		return false;
	}

	std::vector<XrActionSuggestedBinding> bindings;

	for (size_t i = 0; i < count; ++i)
	{
		XrActionSuggestedBinding suggestedBinding;
		suggestedBinding.action = actions[i];
		suggestedBinding.binding = paths[i];
		bindings.emplace_back(suggestedBinding);
	}

	XrInteractionProfileSuggestedBinding suggestedBindings{ XR_TYPE_INTERACTION_PROFILE_SUGGESTED_BINDING };
	suggestedBindings.interactionProfile = path;
	suggestedBindings.suggestedBindings = bindings.data();
	suggestedBindings.countSuggestedBindings = static_cast<uint32_t>(bindings.size());

	return XR_SUCCEEDED(xrSuggestInteractionProfileBindings(instance, &suggestedBindings));
}

ActionSet::ActionSet(XrInstance instance, const char* name, const char* localizedName, uint32_t priority)
{
	m_Name = name;
	m_LocalizedName = localizedName;

	m_Instance = instance;

	// Create an action set.
	{
		XrActionSetCreateInfo actionSetInfo{ XR_TYPE_ACTION_SET_CREATE_INFO };
		strcpy_s(actionSetInfo.actionSetName, name);
		strcpy_s(actionSetInfo.localizedActionSetName, localizedName);
		actionSetInfo.priority = priority;
		CHECK_XRCMD(xrCreateActionSet(m_Instance, &actionSetInfo, &m_Handle));
	}

	// Get the XrPath for the left and right hands - we will use them as subaction paths.
	CHECK_XRCMD(xrStringToPath(m_Instance, "/user/hand/left", &m_SubactionPaths[0]));
	CHECK_XRCMD(xrStringToPath(m_Instance, "/user/hand/right", &m_SubactionPaths[1]));
}

void ActionSet::AddActions(const Session& session, std::vector<ActionDefinition> const& actionDefs)
{
	for (auto const& actionDef : actionDefs)
	{
		m_Actions.insert_or_assign(actionDef.name,
								   Action(session, m_Handle, actionDef.type, m_SubactionPaths.data(),
										  m_SubactionPaths.size(), actionDef.name, actionDef.localizedName));
	}
}

void ActionSet::SuggestBindings(std::string const& profilePath,
								std::vector<std::pair<std::string, std::string>> const& bindings)
{
	XrPath profileXrPath;
	CHECK_XRCMD(xrStringToPath(m_Instance, profilePath.c_str(), &profileXrPath));

	std::vector<XrActionSuggestedBinding> suggestedBindings;
	for (auto p : bindings)
	{
		XrPath path;
		CHECK_XRCMD(xrStringToPath(m_Instance, p.second.c_str(), &path));
		suggestedBindings.push_back({ m_Actions.at(p.first), path });
	}

	m_SuggestedBindings[profileXrPath] = suggestedBindings;
}

void ActionSet::Update(Session const& session)
{
	bool active = false;
	for (auto& p : m_Actions)
		active |= p.second.Update(session);
	m_Connected = active;
}

InputManager::InputManager(XrInstance instance, const Session& session)
{
	m_Instance = instance;

	// We can only suggest bindings for each interaction profile once, but we have multiple
	// action sets. To work around this, we concatenate each of the action sets' suggested
	// bindings so that we only have to suggest them once per profile.
	std::unordered_map<XrPath, std::vector<XrActionSuggestedBinding>> suggestedBindings;
	for (auto& as : GetActionSets())
	{
		for (auto& sb : as.second.GetSuggestedBindings())
		{
			auto const& path = sb.first;
			auto const& bindings = sb.second;
			suggestedBindings[path].insert(suggestedBindings[path].end(), bindings.begin(), bindings.end());
		}
	}

	for (auto& sb : suggestedBindings)
	{
		auto const& path = sb.first;
		auto const& bindings = sb.second;

		XrInteractionProfileSuggestedBinding suggestedXrBindings{ XR_TYPE_INTERACTION_PROFILE_SUGGESTED_BINDING };
		suggestedXrBindings.interactionProfile = path;
		suggestedXrBindings.suggestedBindings = bindings.data();
		suggestedXrBindings.countSuggestedBindings = (uint32_t)bindings.size();
		CHECK_XRCMD(xrSuggestInteractionProfileBindings(m_Instance, &suggestedXrBindings));
	}

	std::vector<XrActionSet> actionSets;
	GetXRActionSets(actionSets);

	if (actionSets.empty())
	{
		Logf("InputManager: No action sets available (TODO)");
		return;
	}

	XrSessionActionSetsAttachInfo attachInfo{ XR_TYPE_SESSION_ACTION_SETS_ATTACH_INFO };
	attachInfo.actionSets = actionSets.data();
	attachInfo.countActionSets = static_cast<uint32_t>(actionSets.size());
	CHECK_XRCMD(xrAttachSessionActionSets(session, &attachInfo));
}

void InputManager::Update(const Session& session)
{
	if (m_ActionSets.empty())
	{
		return;
	}

	XrActiveActionSet activeActionSet;
	auto& actionSet = m_ActionSets.at("default");
	activeActionSet.actionSet = actionSet.GetHandle();
	activeActionSet.subactionPath = XR_NULL_PATH;

	XrActionsSyncInfo syncInfo{ XR_TYPE_ACTIONS_SYNC_INFO };
	syncInfo.activeActionSets = &activeActionSet;
	syncInfo.countActiveActionSets = 1;

	CHECK_XRCMD(xrSyncActions(session, &syncInfo));

	actionSet.Update(session);
}

Action const* InputManager::GetAction(std::string const& setName, std::string const& name) const
{
	auto const& set = GetActionSet(setName);
	if (!set.IsConnected())
		return nullptr;

	return &set.GetAction(name);
}

void InputManager::GetXRActionSets(std::vector<XrActionSet>& actionSets)
{
	actionSets.clear();
	actionSets.reserve(m_ActionSets.size());

	for (auto const& p : m_ActionSets)
		actionSets.emplace_back(p.second.GetHandle());
}
};	// namespace xr
