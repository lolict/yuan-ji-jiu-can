"""
MetaIME (ime-agent) · Core Modules
  manifest         — Capability-Based Security
  intent_parser    — 输入 → 意图
  auto_pilot       — 自动化执行 (多端)
  agent            — 本地 AI 决策引擎 (态射推理)
  channel_router   — 通道路由 (三元仲裁)
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
