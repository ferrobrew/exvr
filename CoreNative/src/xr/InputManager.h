#pragma once

#include <vector>
#include <array>
#include <unordered_map>
#include <memory>
#include <string>
#include <openxr/openxr.h>
#include <cassert>

namespace xr
{
class Action
{
public:
	Action(
		XrSession session, XrActionSet actionSet, XrActionType type,
		XrPath const* subactionPaths, size_t subactionCount,
		std::string const& actionName, std::string const& localizedName);
	operator XrAction() const { return m_Action; }

	bool Update(const class Session& session);
	void LogBinding(const class Session& session) const;

	XrActionType GetType() const { return m_ActionType; }
	XrSpace GetSpace() const { return m_Space; }
	std::string const& GetLocalizedName() const { return m_LocalizedName; }

#define ASSERT_TYPE(type) assert(m_ActionType == type)
	bool IsPressed() const
	{
		ASSERT_TYPE(XR_ACTION_TYPE_BOOLEAN_INPUT);
		return m_Boolean.m_IsPressed;
	};
	bool WasPressed() const
	{
		ASSERT_TYPE(XR_ACTION_TYPE_BOOLEAN_INPUT);
		return IsPressed() && !m_Boolean.m_WasPressed;
	};
	bool IsReleased() const
	{
		ASSERT_TYPE(XR_ACTION_TYPE_BOOLEAN_INPUT);
		return !m_Boolean.m_IsPressed;
	};
	bool WasReleased() const
	{
		ASSERT_TYPE(XR_ACTION_TYPE_BOOLEAN_INPUT);
		return IsReleased() && m_Boolean.m_WasPressed;
	};
	float GetValue() const
	{
		ASSERT_TYPE(XR_ACTION_TYPE_FLOAT_INPUT);
		return m_Float.m_Value;
	};
	XrVector2f GetVector() const
	{
		ASSERT_TYPE(XR_ACTION_TYPE_VECTOR2F_INPUT);
		return m_Vector.m_Value;
	};
	XrPosef GetPose() const
	{
		ASSERT_TYPE(XR_ACTION_TYPE_POSE_INPUT);
		return m_Pose.m_Value;
	};
#undef ASSERT_TYPE

private:
	XrActionType m_ActionType{ XR_ACTION_TYPE_MAX_ENUM };
	XrSpace m_Space{ XR_NULL_HANDLE };
	XrAction m_Action{ XR_NULL_HANDLE };

	union
	{
		struct
		{
			XrVector2f m_Value{ 0.f, 0.f };
		} m_Vector;
		struct
		{
			float m_Value{ 0.f };
		} m_Float;
		struct
		{
			bool m_IsPressed{ false };
			bool m_WasPressed{ false };
		} m_Boolean;
		struct
		{
			XrPosef m_Value{ { 0.f, 0.f, 0.f, 0.f }, { 0.f, 0.f, 0.f } };
		} m_Pose{};
	};

	std::string m_LocalizedName;
};

class ActionSet
{
public:
	struct ActionDefinition
	{
		XrActionType type;
		std::string name;
		std::string localizedName;
	};

	ActionSet(XrInstance instance, const char* name, const char* localizedName, uint32_t priority);

	XrActionSet GetHandle() const { return m_Handle; }
	std::string const& GetName() const { return m_Name; }
	std::string const& GetLocalizedName() const { return m_LocalizedName; }
	bool IsConnected() const { return m_Connected; }

	std::unordered_map<std::string, Action> const& GetActions() const { return m_Actions; }
	Action const& GetAction(std::string const& name) const { return m_Actions.at(name); }
	Action const& operator[](std::string const& name) const { return GetAction(name); }

	void AddActions(const class Session& session, std::vector<ActionDefinition> const& actionDefs);
	void SuggestBindings(std::string const& profilePath, std::vector<std::pair<std::string, std::string>> const& bindings);
	std::unordered_map<XrPath, std::vector<XrActionSuggestedBinding>> const& GetSuggestedBindings() const { return m_SuggestedBindings; }

	void Update(const class Session& session);

private:
	std::string m_Name;
	std::string m_LocalizedName;

	XrInstance m_Instance{ XR_NULL_HANDLE };
	XrActionSet m_Handle{ XR_NULL_HANDLE };
	std::array<XrPath, 2> m_SubactionPaths;

	std::unordered_map<std::string, Action> m_Actions;
	std::unordered_map<XrPath, std::vector<XrActionSuggestedBinding>> m_SuggestedBindings;
	bool m_Connected;
};

class InputManager
{
public:
	InputManager(XrInstance instance, const class Session& session);
	void Update(const class Session& session);

	std::unordered_map<std::string, ActionSet> const& GetActionSets() const { return m_ActionSets; };
	ActionSet const& GetActionSet(std::string const& name) const { return m_ActionSets.at(name); }
	Action const* GetAction(std::string const& setName, std::string const& name) const;

private:
	XrInstance m_Instance{ XR_NULL_HANDLE };
	std::unordered_map<std::string, ActionSet> m_ActionSets;

	void GetXRActionSets(std::vector<XrActionSet>& actionSets);
};
};	// namespace xr
