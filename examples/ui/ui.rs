use bevy::prelude::*;

macro_rules! flex {
    (@layout {$(!color: $color:expr,)? $($field:ident : $content:expr),*} [$mat:expr]) => ({
        let default_style = Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        };
        let default_node = NodeBundle {
            style: Style { $($field : $content,)* .. default_style },
            material: $mat.add(Color::NONE.into()),
            ..Default::default()
        };
        NodeBundle { $(material: $mat.add($color.into()),)? .. default_node }
    });
    (@control text
        [$cmds:expr, $mat:expr, $font:expr]
        $text:expr
    ) => ({
        let text_style = TextStyle {
            color: Color::WHITE,
            font: $font.clone(),
            font_size: 20.0,
        };
        let text_align = TextAlignment { horizontal: HorizontalAlign::Left, ..Default::default() };
        $cmds.spawn_bundle(TextBundle {
            style: Style::default(),
            text: Text::with_section($text, text_style, text_align),
            ..Default::default()
        })
    });
    (@control image
        [$cmds:expr, $mat:expr, $font:expr]
        $image:expr
    ) => (
        $cmds.spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(500.0), Val::Auto),
                ..Default::default()
            },
            material: $mat.add($image),
            ..Default::default()
        })
    );
    (@control layout
        {$($params:tt)*}
        [$cmds:expr, $mat:expr, $font:expr]
        $(
            $control:ident
                $({$($ctrl_params:tt)*})?
                ( $($control_args:tt)* )
        )*
    ) => (
        #[allow(unused_variables)]
        $cmds.spawn_bundle(flex!(@layout {$($params)*} [$mat]))
            .with_children(|parent| {
                $(
                    flex!(
                        @control $control
                            $({$($ctrl_params)*})?
                            [parent, $mat, $font]
                            $($control_args)*
                    );
                )*
            })
    );
    (@control vertical [$($x:tt)*]
        $( $control:ident $({$($params:tt)*})? ( $($control_args:tt)* ) )*
    ) => (
        flex!(
            @control layout {flex_direction: FlexDirection::Column} [$($x)*]
            $( $control $({$($params)*})? ( $($control_args)* ) )*
        )
    );
    (@control horizontal [$($x:tt)*]
        $( $control:ident $({$($params:tt)*})? ( $($control_args:tt)* ) )*
    ) => (
        flex!(
            @control layout {flex_direction: FlexDirection::Row} [$($x)*]
            $( $control $({$($params)*})? ( $($control_args)* ) )*
        )
    );
}

macro_rules! size {
    (@unit px $value:literal) => (Val::Px($value));
    (@unit pct $value:literal) => (Val::Percent($value));
    ($x:literal $x_unit:ident, $y:literal $y_unit:ident) => (
        Size::new(size!(@unit $x_unit $x), size!(@unit $y_unit $y))
    );
}

macro_rules! grey {
    ($value:literal) => (
        Color::rgb($value, $value, $value)
    )
}

macro_rules! col {
    ($r:expr, $g:expr, $b:expr) => (
        Color::rgb($r, $g, $b)
    )
}

/// This example illustrates the various features of Bevy UI.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let px = |f: f32| Val::Px(f);
    let abs_pos = |left: f32, bottom: f32| 
        Rect { left: px(left), bottom: px(bottom), ..Default::default() };

    let bevy_logo = asset_server.load("branding/bevy_logo_dark_big.png");
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // root node
    flex! {
        @control layout {justify_content: JustifyContent::SpaceBetween}
        [commands, materials, font]

        // left vertical fill (border)
        layout { !color: grey!(0.65), size: size!(200.0 px, 100.0 pct), border: Rect::all(px(2.0)) } (
            // left vertical fill (content)
            layout { !color: grey!(0.15), align_items: AlignItems::FlexEnd } (
                text("Text Example")
            )
        )
        // right vertical fill
        layout {!color: grey!(0.15), size: size!(200.0 px, 100.0 pct)} ()
        // absoulte positioning
        layout {
            !color: col!(0.4, 0.4, 1.0),
            size: size!(200.0 px, 200.0 px),
            position_type: PositionType::Absolute,
            position: abs_pos(210.0, 10.0),
            border: Rect::all(px(20.0))
        } (
            layout {!color: col!(0.8, 0.8, 1.0), } ()
        )
        // render order test: reddest in the back, whitest in the front (flex center)
        layout {
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center
        } (
            layout {!color: Color::RED, size: size!(100.0 px, 100.0 px)} (
                layout {
                    !color: col!(1.0, 0.3, 0.3),
                    size: size!(100.0 px, 100.0 px),
                    position_type: PositionType::Absolute,
                    position: abs_pos(20.0, 20.0)
                } ()
                layout {
                    !color: col!(1.0, 0.5, 0.5),
                    size: size!(100.0 px, 100.0 px),
                    position_type: PositionType::Absolute,
                    position: abs_pos(40.0, 40.0)
                } ()
                layout {
                    !color: col!(1.0, 0.7, 0.7),
                    size: size!(100.0 px, 100.0 px),
                    position_type: PositionType::Absolute,
                    position: abs_pos(60.0, 60.0)
                } ()
                // alpha test
                layout {
                    !color: Color::rgba(1.0, 0.9, 0.9, 0.4),
                    size: size!(100.0 px, 100.0 px),
                    position_type: PositionType::Absolute,
                    position: abs_pos(80.0, 80.0)
                } ()
            )
        )
        // bevy logo (flex center)
        layout {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd
        } (
            image(bevy_logo.into())
        )
        // flex layout
        layout {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceEvenly, 
            position_type: PositionType::Absolute,
            position: abs_pos(30.0, 20.0),
            size: size!(300.0 px, 80.0 pct)
        } (
            text("Text Example")
            text("Text Example")
            text("Text test")
            text("Text attempt")
        )
    };
}
