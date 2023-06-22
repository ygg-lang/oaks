use serde::{Deserialize, Serialize};

/// 可视化主题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationTheme {
    /// 主题名称
    pub name: String,
    /// 背景颜色
    pub background_color: String,
    /// 节点样式
    pub node: NodeTheme,
    /// 边样式
    pub edge: EdgeTheme,
    /// 文本样式
    pub text: TextTheme,
    /// 高亮样式
    pub highlight: HighlightTheme,
}

/// 节点主题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeTheme {
    /// 默认填充颜色
    pub fill_color: String,
    /// 默认边框颜色
    pub stroke_color: String,
    /// 边框宽度
    pub stroke_width: f32,
    /// 圆角半径
    pub border_radius: f32,
    /// 阴影配置
    pub shadow: ShadowConfig,
}

/// 边主题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeTheme {
    /// 默认颜色
    pub color: String,
    /// 线宽
    pub width: f32,
    /// 线型 (solid, dashed, dotted)
    pub style: String,
    /// 箭头样式
    pub arrow: ArrowConfig,
}

/// 文本主题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextTheme {
    /// 字体族
    pub font_family: String,
    /// 字体大小
    pub font_size: f32,
    /// 字体颜色
    pub color: String,
    /// 字体粗细
    pub font_weight: String,
}

/// 高亮主题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightTheme {
    /// 选中状态颜色
    pub selected_color: String,
    /// 悬停状态颜色
    pub hover_color: String,
    /// 错误状态颜色
    pub error_color: String,
    /// 警告状态颜色
    pub warning_color: String,
}

/// 阴影配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowConfig {
    /// 是否启用阴影
    pub enabled: bool,
    /// 阴影颜色
    pub color: String,
    /// 阴影偏移X
    pub offset_x: f32,
    /// 阴影偏移Y
    pub offset_y: f32,
    /// 阴影模糊半径
    pub blur_radius: f32,
}

/// 箭头配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrowConfig {
    /// 是否显示箭头
    pub enabled: bool,
    /// 箭头大小
    pub size: f32,
    /// 箭头类型 (triangle, circle, diamond)
    pub arrow_type: String,
}

impl VisualizationTheme {
    /// 转换为渲染配置
    pub fn to_render_config(&self) -> crate::render::RenderConfig {
        crate::render::RenderConfig {
            background_color: self.background_color.clone(),
            node_fill_color: self.node.fill_color.clone(),
            node_stroke_color: self.node.stroke_color.clone(),
            node_stroke_width: self.node.stroke_width as f64,
            edge_color: self.edge.color.clone(),
            edge_width: self.edge.width as f64,
            text_color: self.text.color.clone(),
            text_size: self.text.font_size as f64,
            font_family: self.text.font_family.clone(),
            show_arrows: self.edge.arrow.enabled,
            arrow_size: self.edge.arrow.size as f64,
            ..Default::default()
        }
    }
}

impl Default for VisualizationTheme {
    fn default() -> Self {
        Self::light()
    }
}

impl VisualizationTheme {
    /// 浅色主题
    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),
            background_color: "#FFFFFF".to_string(),
            node: NodeTheme {
                fill_color: "#F8F9FA".to_string(),
                stroke_color: "#DEE2E6".to_string(),
                stroke_width: 1.0,
                border_radius: 4.0,
                shadow: ShadowConfig { enabled: true, color: "rgba(0, 0, 0, 0.1)".to_string(), offset_x: 0.0, offset_y: 2.0, blur_radius: 4.0 },
            },
            edge: EdgeTheme { color: "#6C757D".to_string(), width: 1.5, style: "solid".to_string(), arrow: ArrowConfig { enabled: true, size: 8.0, arrow_type: "triangle".to_string() } },
            text: TextTheme { font_family: "Arial, sans-serif".to_string(), font_size: 12.0, color: "#212529".to_string(), font_weight: "normal".to_string() },
            highlight: HighlightTheme { selected_color: "#007BFF".to_string(), hover_color: "#0056B3".to_string(), error_color: "#DC3545".to_string(), warning_color: "#FFC107".to_string() },
        }
    }

    /// 深色主题
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            background_color: "#1E1E1E".to_string(),
            node: NodeTheme {
                fill_color: "#2D2D30".to_string(),
                stroke_color: "#3E3E42".to_string(),
                stroke_width: 1.0,
                border_radius: 4.0,
                shadow: ShadowConfig { enabled: true, color: "rgba(0, 0, 0, 0.3)".to_string(), offset_x: 0.0, offset_y: 2.0, blur_radius: 4.0 },
            },
            edge: EdgeTheme { color: "#CCCCCC".to_string(), width: 1.5, style: "solid".to_string(), arrow: ArrowConfig { enabled: true, size: 8.0, arrow_type: "triangle".to_string() } },
            text: TextTheme { font_family: "Arial, sans-serif".to_string(), font_size: 12.0, color: "#CCCCCC".to_string(), font_weight: "normal".to_string() },
            highlight: HighlightTheme { selected_color: "#0E639C".to_string(), hover_color: "#1177BB".to_string(), error_color: "#F14C4C".to_string(), warning_color: "#FFCC02".to_string() },
        }
    }

    /// One Light 主题 - 基于 Atom One Light
    pub fn one_light() -> Self {
        Self {
            name: "One Light".to_string(),
            background_color: "#FAFAFA".to_string(),
            node: NodeTheme {
                fill_color: "#FFFFFF".to_string(),
                stroke_color: "#E1E4E8".to_string(),
                stroke_width: 1.0,
                border_radius: 6.0,
                shadow: ShadowConfig { enabled: true, color: "rgba(149, 157, 165, 0.2)".to_string(), offset_x: 0.0, offset_y: 8.0, blur_radius: 24.0 },
            },
            edge: EdgeTheme { color: "#586069".to_string(), width: 1.5, style: "solid".to_string(), arrow: ArrowConfig { enabled: true, size: 8.0, arrow_type: "triangle".to_string() } },
            text: TextTheme { font_family: "SF Pro Display, -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, sans-serif".to_string(), font_size: 12.0, color: "#24292E".to_string(), font_weight: "400".to_string() },
            highlight: HighlightTheme { selected_color: "#0366D6".to_string(), hover_color: "#0256CC".to_string(), error_color: "#D73A49".to_string(), warning_color: "#F66A0A".to_string() },
        }
    }

    /// One Dark Pro 主题 - 基于 Atom One Dark Pro
    pub fn one_dark_pro() -> Self {
        Self {
            name: "One Dark Pro".to_string(),
            background_color: "#282C34".to_string(),
            node: NodeTheme {
                fill_color: "#21252B".to_string(),
                stroke_color: "#3E4451".to_string(),
                stroke_width: 1.0,
                border_radius: 6.0,
                shadow: ShadowConfig { enabled: true, color: "rgba(0, 0, 0, 0.4)".to_string(), offset_x: 0.0, offset_y: 8.0, blur_radius: 24.0 },
            },
            edge: EdgeTheme { color: "#ABB2BF".to_string(), width: 1.5, style: "solid".to_string(), arrow: ArrowConfig { enabled: true, size: 8.0, arrow_type: "triangle".to_string() } },
            text: TextTheme { font_family: "SF Mono, Monaco, Inconsolata, Roboto Mono, monospace".to_string(), font_size: 12.0, color: "#ABB2BF".to_string(), font_weight: "400".to_string() },
            highlight: HighlightTheme { selected_color: "#61AFEF".to_string(), hover_color: "#528BFF".to_string(), error_color: "#E06C75".to_string(), warning_color: "#E5C07B".to_string() },
        }
    }

    /// GitHub 主题
    pub fn github() -> Self {
        Self {
            name: "GitHub".to_string(),
            background_color: "#FFFFFF".to_string(),
            node: NodeTheme {
                fill_color: "#F6F8FA".to_string(),
                stroke_color: "#D0D7DE".to_string(),
                stroke_width: 1.0,
                border_radius: 6.0,
                shadow: ShadowConfig { enabled: true, color: "rgba(31, 35, 40, 0.04)".to_string(), offset_x: 0.0, offset_y: 1.0, blur_radius: 0.0 },
            },
            edge: EdgeTheme { color: "#656D76".to_string(), width: 1.0, style: "solid".to_string(), arrow: ArrowConfig { enabled: true, size: 6.0, arrow_type: "triangle".to_string() } },
            text: TextTheme { font_family: "-apple-system, BlinkMacSystemFont, Segoe UI, Helvetica, Arial, sans-serif".to_string(), font_size: 12.0, color: "#24292F".to_string(), font_weight: "400".to_string() },
            highlight: HighlightTheme { selected_color: "#0969DA".to_string(), hover_color: "#0860CA".to_string(), error_color: "#CF222E".to_string(), warning_color: "#9A6700".to_string() },
        }
    }
}
