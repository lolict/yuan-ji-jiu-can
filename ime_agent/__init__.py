"""
MetaIME (ime-agent) · Core Modules — Python Package
"""

from .manifest import Manifest, CapabilityToken, CapabilityScope
from .intent_parser import IntentParser, Intent, IntentType
from .auto_pilot import AutoPilot, AutoAction, ActionType, Platform
from .agent import LocalAgent, Decision, DecisionStatus, KnowledgeNode
from .channel_router import ChannelRouter, RouteDecision, Channel

__all__ = [
    "Manifest", "CapabilityToken", "CapabilityScope",
    "IntentParser", "Intent", "IntentType",
    "AutoPilot", "AutoAction", "ActionType", "Platform",
    "LocalAgent", "Decision", "DecisionStatus", "KnowledgeNode",
    "ChannelRouter", "RouteDecision", "Channel",
]
