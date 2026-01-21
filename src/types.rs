/// Represents the schema for the UI structure and additional components.
/// The functionality of this element is currently not implemented.
///
/// # Variants
///
/// - `Ratatui` - Port for Ratatui crate.
/// - `AnyOther { name, url }` - Port for any other UI framework.
///
/// # Syntax
///
/// Use on top level of the .gl file
/// ```glyph
/// @language ratatui
/// ```
#[derive(Debug, Clone)]
pub enum Language {
    Ratatui,
    AnyOther {
        name: String,
        url: String,
    }
}

/// Represents the root UI structure.
///
/// # Fields
///
/// - `language` (`Language`) - The used schema.
/// - `root` (`Element`) - The root element of the UI.
#[derive(Debug, Clone)]
pub struct UI {
    pub language: Language,
    pub root: Element,
}

/// Represents any UI element.
///
/// # Variants
///
/// - `Form(Form)` - Application form.
/// - `Panel(Panel)` - Container for grouping elements.
/// - `Label(Label)` - Text label for an element.
/// - `TextInput(TextInput)` - Input field for text.
/// - `Button(Button)` - Clickable button.
/// - `Checkbox(Checkbox)` - Checkbox input.
/// - `RadioGroup(RadioGroup)` - Group of radio buttons.
/// - `Radio(Radio)` - Contained radio button.
/// - `Dropdown(Dropdown)` - Dropdown menu.
/// - `Grid(Grid)` - Grid layout.
/// - `Column(Column)` - Grid column.
/// - `Modal(Modal)` - Modal dialog.
/// - `Tabs(Tabs)` - Tabbed interface.
/// - `Tab(Tab)` - Single tab.
/// - `Custom(CustomElement)` - Custom UI element.
#[derive(Debug, Clone)]
pub enum Element {
    Form(Form),
    Panel(Panel),
    Label(Label),
    TextInput(TextInput),
    Button(Button),
    Checkbox(Checkbox),
    RadioGroup(RadioGroup),
    Radio(Radio),
    Dropdown(Dropdown),
    Grid(Grid),
    Column(Column),
    Modal(Modal),
    Tabs(Tabs),
    Tab(Tab),
    Custom(CustomElement),
}

/// Represents arrangement options for children inside a container.
///
/// # Variants
///
/// - `LeftToRight` - Arrange children from left to right.
/// - `RightToLeft` - Arrange children from right to left.
/// - `TopToBottom` - Arrange children from top to bottom.
/// - `BottomToTop` - Arrange children from bottom to top.
/// - `FreeForm` - Allow children to be positioned freely
///                and force usage of absolute positions.
#[derive(Debug, Clone)]
pub enum Layout {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
    FreeForm,
}

/// Represents a single size constraint for UI elements.
///
/// # Variants
///
/// - `Auto` - The size is determined automatically - equivalent to `None`.
/// - `Fixed(u32)` - The size is a fixed value.
/// - `Percentage(u32)` - The size is a percentage of the parent element's size.
/// ```
#[derive(Debug, Clone)]
pub enum SizeConstraint {
    Auto,
    Fixed(u32),
    Percentage(u32),
}

/// Represents size constraints for UI elements.
///
/// # Fields
///
/// - `width` (`SizeConstraint`) - Width of the element.
/// - `height` (`SizeConstraint`) - Height of the element.
/// - `left` (`SizeConstraint`) - Left margin of the element.
/// - `top` (`SizeConstraint`) - Top margin of the element.
#[derive(Debug, Clone)]
pub struct SizeConstraints {
    pub width: SizeConstraint,
    pub height: SizeConstraint,
    pub left: SizeConstraint,
    pub top: SizeConstraint,
}

/// Represents the main application form.
///
/// # Fields
///
/// - `title` (`String`) - Title of the form.
/// - `layout` (`Layout`) - Layout of the form.
/// - `children` (`Vec<Element>`) - Children elements of the form.
#[derive(Debug, Clone)]
pub struct Form {
    pub title: String,
    pub layout: Layout,
    pub children: Vec<Element>,
}

/// Represents margins around a UI element.
#[derive(Debug, Clone)]
pub struct Margins {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}

/// Represents a panel container for grouping elements.
#[derive(Debug, Clone)]
pub struct Panel {
    pub title: String,
    pub layout: Layout,
    pub children: Vec<Element>,
    pub size_constraints: SizeConstraints,
    pub margins: Margins,
}

/// Represents a standalone text label.
#[derive(Debug, Clone)]
pub struct Label {
    pub text: String,
    pub word_wrap: bool,
    pub size_constraints: SizeConstraints,
    pub margins: Margins,
}

/// Represents a text input field.
#[derive(Debug, Clone)]
pub struct TextInput {
    pub placeholder: String,
    pub default_text: String,
    pub size_constraints: SizeConstraints,
    pub margins: Margins,
    pub read_only: bool,
}

/// Represents a clickable button.
#[derive(Debug, Clone)]
pub struct Button {
    pub text: String,
    pub size_constraints: SizeConstraints,
    pub margins: Margins,
}

/// Represents a checkbox input.
#[derive(Debug, Clone)]
pub struct Checkbox {
    pub label: String,
    pub checked: bool,
    pub size_constraints: SizeConstraints,
    pub margins: Margins,
}

/// Represents a single radio button.
#[derive(Debug, Clone)]
pub struct Radio {
    pub label: String,
    pub value: String,
    pub margins: Margins,
}

/// Represents a group of radio buttons.
#[derive(Debug, Clone)]
pub struct RadioGroup {
    pub children: Vec<Radio>,
    pub selected_radio: String,
    pub size_constraints: SizeConstraints,
    pub margins: Margins,
}

/// Represents one of the possible values for a dropdown option.
///
/// # Variants
///
/// - `StringValue(String)` - String value.
/// - `NumberValue(i64)` - Integer value.
/// - `FloatValue(f64)` - Floating-point value.
/// - `BoolValue(bool)` - Boolean value.
#[derive(Debug, Clone)]
pub enum DropdownOptionValue {
    StringValue(String),
    NumberValue(i64),
    FloatValue(f64),
    BoolValue(bool),
}

/// Represents a single option in a dropdown menu.
#[derive(Debug, Clone)]
pub struct DropdownOption {
    pub label: String,
    pub value: DropdownOptionValue,
}

/// Represents a dropdown menu.
#[derive(Debug, Clone)]
pub struct Dropdown {
    pub options: Vec<DropdownOption>,
    pub selected_option: String,
    pub size_constraints: SizeConstraints,
    pub margins: Margins,
}

/// Represents a grid view.
#[derive(Debug, Clone)]
pub struct Grid {
    pub columns: Vec<Column>,
    pub size_constraints: SizeConstraints,
    pub margins: Margins,
}

/// Represents a single column in a grid.
#[derive(Debug, Clone)]
pub struct Column {
    pub title: String,
    pub width: SizeConstraint,
}

/// Represents a modal dialog.
#[derive(Debug, Clone)]
pub struct Modal {
    pub title: String,
    pub children: Vec<Element>,
    pub size_constraints: SizeConstraints,
}

/// Represents the position of tabs in a tab control.
#[derive(Debug, Clone)]
pub enum TabPosition {
    Top,
    Bottom,
    Left,
    Right,
}

/// Represents a tab control.
#[derive(Debug, Clone)]
pub struct Tabs {
    pub children: Vec<Tab>,
    pub selected_tab: String,
    pub tab_position: TabPosition,
    pub size_constraints: SizeConstraints,
    pub margins: Margins,
}

/// Represents a single tab.
#[derive(Debug, Clone)]
pub struct Tab {
    pub title: String,
    pub children: Vec<Element>,
}

pub trait CustomUIElement: std::fmt::Debug + Send + Sync {
    fn size_constraints(&self) -> SizeConstraints;
    fn margins(&self) -> Margins;
    fn render(&self);
    fn clone_box(&self) -> Box<dyn CustomUIElement>;
}

#[derive(Debug)]
pub struct CustomElement {
    pub implementation: Box<dyn CustomUIElement>,
}

impl Clone for CustomElement {
    fn clone(&self) -> Self {
        CustomElement {
            implementation: self.implementation.clone_box(),
        }
    }
}