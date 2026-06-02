# 布罗卡区语义规划层模块
# Broca semantic planning layer module
# 将符号决策包转化为抽象语义骨架，定义信息结构和关键实体
# Converts symbolic decision packages into abstract semantic skeletons, defining information structure and key entities

class SemanticPlanner:
    """
    语义规划器
    Semantic planner
    根据意图和事实构建语义骨架
    Builds semantic skeleton based on intent and facts
    """
    
    def plan(self, decision_package: dict) -> dict:
        """
        规划语义骨架
        Plan semantic skeleton
        
        Args:
            decision_package: 符号决策包 / Symbolic decision package
            
        Returns:
            语义骨架，包含意图、槽位、风格等信息 / Semantic skeleton
        """
        intent = decision_package.get("intent", "Unknown")
        fact = decision_package.get("fact")
        style = decision_package.get("style", "normal")
        fallback = decision_package.get("fallback_action")
        
        skeleton = {
            "intent": intent,
            "slots": {
                "subject": fact.get("subject") if fact else None,
                "attribute": fact.get("attribute") if fact else None,
                "value": fact.get("value") if fact else None,
                "description": fact.get("description") if fact else None,
            },
            "style": style,
            "fallback": fallback
        }
        
        # 根据意图添加额外规划信息 / Add extra planning info based on intent
        if intent == "AskFact" and fact:
            skeleton["structure"] = "factual_statement"
        elif intent == "ExpressOpinion":
            skeleton["structure"] = "opinion_expression"
        elif intent == "Chat":
            skeleton["structure"] = "casual_conversation"
        elif intent == "TestBoundary":
            skeleton["structure"] = "defensive_response"
        else:
            skeleton["structure"] = "general_response"
        
        return skeleton