pub mod icons5;

use crate::render::Renderer;

#[derive(Clone, Copy)]
pub enum InternalIcon {
    ArrowLeft,
    ArrowRight,
}

#[derive(Clone, Copy)]
pub enum IconKind {
    // Arrows //
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    SolidArrowLeft,
    SolidArrowRight,
    SolidArrowUp,
    SolidArrowDown,
    SmallArrowLeft,
    SmallArrowRight,
    SmallArrowUp,
    SmallArrowDown,

    // Geometric shapes //
    Square,
    SmallSquare,
    Circle,

    // Standard UI //
    Bars,
    BarsV,
    Check,
    List,
    ListFlipped,
    ThreeDotsV,
    Frame,
    FunnelDown,
    FunnelUp,

    // Actions //
    ExpandH,
    ExpandV,
    ExpandLeftToRight,
    ExpandRightToLeft,
    CollapseH,
    CollapseV,
    CollapseLeftRoRight,
    CollapseRightToLeft,
    AimCenter,
    ConnectUp,
    ConnectRight,
    ConnectLeft,
    ConnectDown,
    MoreH,
    MoreV,

    // Symbols //
    Heart,
    SolidHeart,
    Cross,
    Hashtag,
    Diamond,
    Smile,

    // Math //
    Plus,
    Minus,
    SquareBrackets,
    SmallPlus,
    SmallCross,
    LineV,
    Equal,

    // Music //
    MusicNote,
    Pause,

    // Unsorted //
    LinesTiltLeft,
    LinesTiltRight,
    BarsRise,
    BarsFall,
    BracketRight,
    BracketLeft,
    BracketUp,
    BracketDown,
    Brackets,
    DotInBrackets,
    Fork,
    Flag,
    Slash,
    Backslash,
    StairsUp,
    StairsDown,
    SnakeCw,
    SnakeCcw,
    SpiralCw,
    SpiralCcw,
    BorderLeft,
    BorderRight,
    BorderV,
    BorderH,
    OutlinedDot,
    Human,
    Translucent,
    Building,
    Interconnection,
    Jar,
    Tare,
    Ladder,

    Ascent,
    Descent,

    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,

    BoundTopLeft,
    BoundTopRight,
    BoundBottomRight,
    BoundBottomLeft,

    // Dice (?) //
    Dot,
    TwoDots,
    ThreeDots,
    FourDots,
    FiveDots,
    SixDots,

    EightDots,
    NineDots,

    // Nodes //
    NodeSingle,
    NodeAngle,
    NodeH,
    NodeAll,

    // Hardware //
    Power,
    Contact,

    // Computer //
    Cursor,
    Terminal,
    Underline,
    Overline,
    Bin,
    Directory,
    File,
}

#[derive(Clone, Copy)]
pub struct IconData<'a> {
    pub size: u32,
    pub data: &'a [u8],
}

impl<'a> IconData<'a> {
    pub fn new(size: u32, data: &'a [u8]) -> Self {
        Self { size, data }
    }
}

// pub trait IntoIcon<R: Renderer> {
//     fn into_icon<'a>(self) -> IconData<'a>;
// }

pub trait InternalIconSet<R: Renderer> {
    fn internal<'a>(icon: InternalIcon) -> IconData<'a>;
}

pub trait IconSet {
    const SIZE: u32;

    fn pick(&self, kind: IconKind) -> Option<IconData<'_>>;
}

#[macro_export]
macro_rules! make_icon_set {
    ($vis: vis $name: ident: $size: literal {
        $($data_name: ident: $method_name: ident = $data: expr),*
        $(,)?
    }) => {
        $vis struct $name;

        impl $crate::icons::IconSet for $name {
            const SIZE: u32 = $size;

            fn pick(&self, kind: $crate::icons::IconKind) -> Option<$crate::icons::IconData<'_>> {
                match kind {
                    $($crate::icons::IconKind::$data_name => Some(self.$method_name()),)*
                    _ => None
                }
            }
        }

        impl $name {
            $(
                #[allow(non_upper_case_globals)]
                const $data_name: &'static [u8] = $data;

                #[inline]
                pub fn $method_name(&self) -> $crate::icons::IconData<'_> {
                    $crate::icons::IconData::new(<Self as $crate::icons::IconSet>::SIZE, Self::$data_name)
                }
            )*
        }
    };
}

pub use make_icon_set;
