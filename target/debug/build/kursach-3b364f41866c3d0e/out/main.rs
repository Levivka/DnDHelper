# [allow (non_snake_case)] # [allow (non_camel_case_types)] # [allow (clippy :: style)] # [allow (clippy :: complexity)] # [allow (unused_braces , unused_parens)] # [allow (clippy :: erasing_op)] # [allow (clippy :: approx_constant)] # [allow (clippy :: eq_op)] # [allow (clippy :: cmp_owned)] # [allow (clippy :: redundant_clone)] # [allow (clippy :: overly_complex_bool_expr)] mod slint_generatedMainWindow {
     use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerColorSchemeSelector_68 {
         r#dark_color_scheme : sp :: Property < bool > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow >> , }
     impl InnerColorSchemeSelector_68 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . dark_color_scheme ()) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerFluentPalette_69 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow >> , }
     impl InnerFluentPalette_69 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerStyleMetrics_70 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow >> , }
     impl InnerStyleMetrics_70 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct Innerinput_root_60 {
         r#root_60 : sp :: r#BasicBorderRectangle , r#text_visibility_62 : sp :: r#Clip , r#text_63 : sp :: r#Text , r#textInput_65 : sp :: r#TextInput , r#touchArea_66 : sp :: r#TouchArea , r#root_60_empty_61_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_60_empty_61_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_60_empty_61_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_60_empty_64_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_60_empty_64_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_60_empty_64_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_60_height : sp :: Property < sp :: LogicalLength > , r#root_60_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_60_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_60_text_63_visible : sp :: Property < bool > , r#root_60_text_visibility_62_height : sp :: Property < sp :: LogicalLength > , r#root_60_text_visibility_62_width : sp :: Property < sp :: LogicalLength > , r#root_60_text_visibility_62_x : sp :: Property < sp :: LogicalLength > , r#root_60_text_visibility_62_y : sp :: Property < sp :: LogicalLength > , r#root_60_touchArea_66_x : sp :: Property < sp :: LogicalLength > , r#root_60_touchArea_66_y : sp :: Property < sp :: LogicalLength > , r#root_60_width : sp :: Property < sp :: LogicalLength > , r#root_60_x : sp :: Property < sp :: LogicalLength > , r#root_60_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , Innerinput_root_60 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl Innerinput_root_60 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_61_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_height }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_61_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 16f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_61_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_64_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_height }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_64_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 16f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_64_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_61_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) + (({
                         * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_64_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_61_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) + (({
                         * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_64_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_63_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((! ({
                         * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get ()) && (((({
                         * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from (""))))) {
                         true }
                     else {
                         (false) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_visibility_62 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_63_visible }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294871583f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Zed Mono")) as sp :: SharedString }
            ) ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_61_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64) - (16f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294871583f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Zed Mono")) as sp :: SharedString }
            ) ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_64_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (4286611584f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64) - (16f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#touchArea_66 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#touchArea_66 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_touchArea_66_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_touchArea_66_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_visibility_62 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_visibility_62 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_visibility_62 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_visibility_62 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_visibility_62 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#input_type) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#touchArea_66 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#touchArea_66 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_64_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ((((({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_width }
                ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64) - (16f64 as f64)) as sp :: Coord , 16f64 as sp :: Coord , ({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_64_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 3u32 => (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_touchArea_66_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_touchArea_66_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 4u32 => (({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_61_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ((((({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_width }
                ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64) - (16f64 as f64)) as sp :: Coord , 16f64 as sp :: Coord , ({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_empty_61_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 4u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (4u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnertextBox_root_67 {
         r#root_67 : sp :: r#Text , r#root_67_x : sp :: Property < sp :: LogicalLength > , r#root_67_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnertextBox_root_67 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnertextBox_root_67 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             ({
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerMainWindow {
         r#root_1 : sp :: r#WindowItem , r#loginForm_visibility_2 : sp :: r#Clip , r#loginForm_3 : sp :: r#Rectangle , r#login_8 : sp :: r#BasicBorderRectangle , r#touchArea_9 : sp :: r#TouchArea , r#text_10 : sp :: r#Text , r#register_12 : sp :: r#BasicBorderRectangle , r#touchArea_13 : sp :: r#TouchArea , r#text_14 : sp :: r#Text , r#profileForm_visibility_15 : sp :: r#Clip , r#profileForm_16 : sp :: r#Rectangle , r#sidedownMenu_visibility_18 : sp :: r#Clip , r#sidedownMenu_19 : sp :: r#BorderRectangle , r#buttons_20 : sp :: r#Empty , r#profile_21 : sp :: r#BasicBorderRectangle , r#touchArea_22 : sp :: r#TouchArea , r#text_23 : sp :: r#Text , r#image_24 : sp :: r#ImageItem , r#notes_25 : sp :: r#BasicBorderRectangle , r#touchArea_26 : sp :: r#TouchArea , r#text_27 : sp :: r#Text , r#image_28 : sp :: r#ImageItem , r#charlist_29 : sp :: r#BasicBorderRectangle , r#touchArea_30 : sp :: r#TouchArea , r#text_31 : sp :: r#Text , r#image_32 : sp :: r#ImageItem , r#dices_33 : sp :: r#BasicBorderRectangle , r#touchArea_34 : sp :: r#TouchArea , r#text_35 : sp :: r#Text , r#image_36 : sp :: r#ImageItem , r#settings_37 : sp :: r#BasicBorderRectangle , r#touchArea_38 : sp :: r#TouchArea , r#text_39 : sp :: r#Text , r#image_40 : sp :: r#ImageItem , r#profileLT_visibility_41 : sp :: r#Clip , r#profileLT_42 : sp :: r#Empty , r#profileInfo_43 : sp :: r#Empty , r#stats_46 : sp :: r#Empty , r#notesLT_49 : sp :: r#Empty , r#noteButtonLT_51 : sp :: r#Empty , r#addNote_52 : sp :: r#BasicBorderRectangle , r#touchArea_53 : sp :: r#TouchArea , r#text_54 : sp :: r#Text , r#flickable_56 : sp :: r#Flickable , r#_viewport_57 : sp :: r#Empty , r#username_5 : Innerinput_root_60 , r#password_6 : Innerinput_root_60 , r#nickname_44 : InnertextBox_root_67 , r#id_45 : InnertextBox_root_67 , r#Characters_47 : InnertextBox_root_67 , r#adventures_48 : InnertextBox_root_67 , r#textbox_50 : InnertextBox_root_67 , r#textbox_55 : InnertextBox_root_67 , r#root_1_addNote_52_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_addNote_52_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_buttons_20_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_buttons_20_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_buttons_20_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_charlist_29_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_charlist_29_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_charlist_29_y : sp :: Property < sp :: LogicalLength > , r#root_1_charlist_selected : sp :: Property < bool > , r#root_1_dices_33_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_dices_33_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_dices_33_y : sp :: Property < sp :: LogicalLength > , r#root_1_dices_selected : sp :: Property < bool > , r#root_1_flickable_56_x : sp :: Property < sp :: LogicalLength > , r#root_1_image_24_horizontal_stretch : sp :: Property < f32 > , r#root_1_image_24_max_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_24_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_24_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_28_horizontal_stretch : sp :: Property < f32 > , r#root_1_image_28_max_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_28_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_28_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_32_horizontal_stretch : sp :: Property < f32 > , r#root_1_image_32_max_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_32_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_32_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_36_horizontal_stretch : sp :: Property < f32 > , r#root_1_image_36_max_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_36_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_36_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_40_horizontal_stretch : sp :: Property < f32 > , r#root_1_image_40_max_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_40_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_image_40_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_inputsLT_4_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_inputsLT_4_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_inputsLT_4_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_login_8_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_login_8_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_login_checked : sp :: Property < bool > , r#root_1_loginForm_3_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_loginForm_3_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_loginForm_3_x : sp :: Property < sp :: LogicalLength > , r#root_1_loginForm_3_y : sp :: Property < sp :: LogicalLength > , r#root_1_loginForm_visibility_2_height : sp :: Property < sp :: LogicalLength > , r#root_1_loginForm_visibility_2_width : sp :: Property < sp :: LogicalLength > , r#root_1_loginForm_visibility_2_x : sp :: Property < sp :: LogicalLength > , r#root_1_loginForm_visibility_2_y : sp :: Property < sp :: LogicalLength > , r#root_1_loginLT_7_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_loginLT_7_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_loginLT_7_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_name : sp :: Property < i32 > , r#root_1_note : sp :: Property < sp :: ModelRc < (sp :: SharedString , sp :: SharedString , i32 ,) > > , r#root_1_note_list : sp :: Property < sp :: ModelRc < i32 > > , r#root_1_noteButtonLT_51_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_noteButtonLT_51_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_noteButtonLT_51_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_noteButtonLT_51_x : sp :: Property < sp :: LogicalLength > , r#root_1_notes_25_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_notes_25_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_notes_25_y : sp :: Property < sp :: LogicalLength > , r#root_1_notes_ammount : sp :: Property < i32 > , r#root_1_notes_selected : sp :: Property < bool > , r#root_1_notesLT_49_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_notesLT_49_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_notesLT_49_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_profile_21_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_profile_21_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_profile_21_y : sp :: Property < sp :: LogicalLength > , r#root_1_profile_selected : sp :: Property < bool > , r#root_1_profileForm_16_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_profileForm_16_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_profileForm_16_x : sp :: Property < sp :: LogicalLength > , r#root_1_profileForm_16_y : sp :: Property < sp :: LogicalLength > , r#root_1_profileForm_visibility_15_height : sp :: Property < sp :: LogicalLength > , r#root_1_profileForm_visibility_15_width : sp :: Property < sp :: LogicalLength > , r#root_1_profileForm_visibility_15_x : sp :: Property < sp :: LogicalLength > , r#root_1_profileForm_visibility_15_y : sp :: Property < sp :: LogicalLength > , r#root_1_profileInfo_43_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_profileInfo_43_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_profileInfo_43_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_profileInfo_43_x : sp :: Property < sp :: LogicalLength > , r#root_1_profileLT_42_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_profileLT_42_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_profileLT_42_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_profileLT_visibility_41_height : sp :: Property < sp :: LogicalLength > , r#root_1_profileLT_visibility_41_width : sp :: Property < sp :: LogicalLength > , r#root_1_profileLT_visibility_41_x : sp :: Property < sp :: LogicalLength > , r#root_1_profileLT_visibility_41_y : sp :: Property < sp :: LogicalLength > , r#root_1_register_12_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_register_12_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_registerBox_11_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_registerBox_11_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_registerBox_11_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_settings_37_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_settings_37_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_settings_37_y : sp :: Property < sp :: LogicalLength > , r#root_1_settings_selected : sp :: Property < bool > , r#root_1_sidedownLT_17_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_sidedownLT_17_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_sidedownLT_17_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_sidedownMenu_19_x : sp :: Property < sp :: LogicalLength > , r#root_1_sidedownMenu_visibility_18_height : sp :: Property < sp :: LogicalLength > , r#root_1_sidedownMenu_visibility_18_width : sp :: Property < sp :: LogicalLength > , r#root_1_sidedownMenu_visibility_18_x : sp :: Property < sp :: LogicalLength > , r#root_1_sidedownMenu_visibility_18_y : sp :: Property < sp :: LogicalLength > , r#root_1_stats_46_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_stats_46_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_stats_46_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_stats_46_x : sp :: Property < sp :: LogicalLength > , r#root_1_text_10_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_10_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_10_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_10_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_14_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_14_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_14_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_14_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_23_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_23_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_23_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_23_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_27_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_27_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_27_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_27_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_31_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_31_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_31_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_31_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_35_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_35_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_35_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_35_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_39_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_39_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_39_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_39_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_54_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_54_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_text_54_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_text_54_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_textbox_55_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_textbox_55_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_textbox_55_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_textbox_55_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_13_x : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_13_y : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_22_x : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_22_y : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_26_x : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_26_y : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_30_x : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_30_y : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_34_x : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_34_y : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_38_x : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_38_y : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_53_x : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_53_y : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_9_x : sp :: Property < sp :: LogicalLength > , r#root_1_touchArea_9_y : sp :: Property < sp :: LogicalLength > , r#root_1_x : sp :: Property < sp :: LogicalLength > , r#root_1_y : sp :: Property < sp :: LogicalLength > , r#root_1_addNote : sp :: Callback < () , () > , r#root_1_charlist_select : sp :: Callback < () , () > , r#root_1_dices_select : sp :: Callback < () , () > , r#root_1_login_check : sp :: Callback < (sp :: SharedString , sp :: SharedString ,) , () > , r#root_1_notes_select : sp :: Callback < () , () > , r#root_1_profile_select : sp :: Callback < () , () > , r#root_1_settings_select : sp :: Callback < () , () > , repeater0 : sp :: Repeater < InnerComponent_textbox_58 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerMainWindow >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , globals : Globals_MainWindow , window_adapter_ : sp :: OnceCell < sp :: WindowAdapterRc > , }
     impl InnerMainWindow {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_note_list }
                    ) . apply_pin (_self) . get ()) as _ }
                 }
            ) ;
             Innerinput_root_60 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#username_5 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 5u32 - 1 , tree_index_of_first_child + 9u32 - 1) ;
             Innerinput_root_60 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#password_6 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 6u32 - 1 , tree_index_of_first_child + 13u32 - 1) ;
             InnertextBox_root_67 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#nickname_44 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 49u32 - 1 , tree_index_of_first_child + 51u32 - 1) ;
             InnertextBox_root_67 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#id_45 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 50u32 - 1 , tree_index_of_first_child + 51u32 - 1) ;
             InnertextBox_root_67 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#Characters_47 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 51u32 - 1 , tree_index_of_first_child + 53u32 - 1) ;
             InnertextBox_root_67 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#adventures_48 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 52u32 - 1 , tree_index_of_first_child + 53u32 - 1) ;
             InnertextBox_root_67 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#textbox_50 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 53u32 - 1 , tree_index_of_first_child + 56u32 - 1) ;
             InnertextBox_root_67 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#textbox_55 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 59u32 - 1 , tree_index_of_first_child + 60u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_addNote_52_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + (sp :: Item :: layout_info (({
                         InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                     + {
                         * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_addNote_52_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + (sp :: Item :: layout_info (({
                         InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                     + {
                         * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_68 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_68 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4280032284f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294638330f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_buttons_20_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_21_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 57.142857142857146f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 57.142857142857146f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_25_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 57.142857142857146f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 57.142857142857146f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_29_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 57.142857142857146f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 57.142857142857146f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_33_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 57.142857142857146f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 57.142857142857146f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_37_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 57.142857142857146f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 57.142857142857146f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : 400f64 as _ , r#spacing : ((400f64 as f64) * (0.02f64 as f64)) as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_buttons_20_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_21_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 57.142857142857146f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 57.142857142857146f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_25_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 57.142857142857146f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 57.142857142857146f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_29_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 57.142857142857146f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 57.142857142857146f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_33_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 57.142857142857146f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 57.142857142857146f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_37_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 57.142857142857146f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 57.142857142857146f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , ((400f64 as f64) * (0.02f64 as f64)) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_buttons_20_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_21_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_25_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_29_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_33_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_37_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_29_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_32_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_32_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_32_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_32_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_29_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_32 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_33_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_36_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_36_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_36_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_36_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_33_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_36 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (600f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_24_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_24_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_24_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_24_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_28_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_28 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_28_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_28 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_28_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_28 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_28_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_28 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_32_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_32 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_32_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_32 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_32_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_32 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_32_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_32 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_36_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_36 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_36_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_36 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_36_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_36 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_36_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_36 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_40_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_40 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_40_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_40 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_40_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_40 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_40_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_40 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_inputsLT_4_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
                                 + {
                                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((600f64 as f64) * (0.09f64 as f64)) as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ((600f64 as f64) * (0.09f64 as f64)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
                                 + {
                                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((600f64 as f64) * (0.09f64 as f64)) as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ((600f64 as f64) * (0.09f64 as f64)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = ((- 600f64 as f64) * (0.4f64 as f64)) as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : 600f64 as _ , r#spacing : ((600f64 as f64) * (0.02f64 as f64)) as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_inputsLT_4_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
                             + {
                                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((400f64 as f64) * (0.9f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((400f64 as f64) * (0.9f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
                             + {
                                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((400f64 as f64) * (0.9f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((400f64 as f64) * (0.9f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = ((400f64 as f64) * (0.04f64 as f64)) as _ ;
                         the_struct . r#end = ((400f64 as f64) * (0.04f64 as f64)) as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_inputsLT_4_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
                             + {
                                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((600f64 as f64) * (0.09f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((600f64 as f64) * (0.09f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
                             + {
                                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((600f64 as f64) * (0.09f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((600f64 as f64) * (0.09f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , ((600f64 as f64) * (0.02f64 as f64)) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = ((- 600f64 as f64) * (0.4f64 as f64)) as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_3_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_16_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notesLT_49_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_3_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_16_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notesLT_49_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_login_8_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_login_8_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_3_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_inputsLT_4_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginLT_7_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_registerBox_11_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_3_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_inputsLT_4_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginLT_7_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_registerBox_11_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginLT_7_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_login_8_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 85.71428571428571f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 85.71428571428571f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = ((- 600f64 as f64) * (((- 20f64 as f64) * (0.01f64 as f64)) as f64)) as _ ;
                             the_struct . r#end = ((- 400f64 as f64) * (((- 20f64 as f64) * (0.01f64 as f64)) as f64)) as _ ;
                             the_struct }
                         as _ , r#size : 600f64 as _ , r#spacing : 8f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginLT_7_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_login_8_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = ((- 400f64 as f64) * (((- 20f64 as f64) * (0.01f64 as f64)) as f64)) as _ ;
                         the_struct . r#end = ((- 400f64 as f64) * (((- 20f64 as f64) * (0.01f64 as f64)) as f64)) as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginLT_7_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_login_8_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 85.71428571428571f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 85.71428571428571f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = ((- 600f64 as f64) * (((- 20f64 as f64) * (0.01f64 as f64)) as f64)) as _ ;
                         the_struct . r#end = ((- 400f64 as f64) * (((- 20f64 as f64) * (0.01f64 as f64)) as f64)) as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_note_list }
            ) . apply_pin (_self) . set ({
                 (sp :: ModelRc :: new (sp :: VecModel :: < i32 > :: from (sp :: vec ! [1f64 as _ , 2f64 as _ , 4f64 as _ , 10f64 as _ , 213f64 as _]))) as sp :: ModelRc < i32 > }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_noteButtonLT_51_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_addNote_52_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((400f64 as f64) * (0.1f64 as f64)) as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ((400f64 as f64) * (0.1f64 as f64)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notesLT_49_layout_cache }
                        ) . apply_pin (_self) . get () [3usize] as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_noteButtonLT_51_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_addNote_52_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((400f64 as f64) * (0.1f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((400f64 as f64) * (0.1f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = ((400f64 as f64) * (0.9f64 as f64)) as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_noteButtonLT_51_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_addNote_52_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((400f64 as f64) * (0.1f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((400f64 as f64) * (0.1f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_25_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_28_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_28_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_28_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_28_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_25_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_28 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notesLT_49_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Start as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
                             + {
                                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_noteButtonLT_51_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_56 }
                                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((600f64 as f64) * (0.75f64 as f64)) as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ((600f64 as f64) * (0.75f64 as f64)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : 600f64 as _ , r#spacing : 1f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notesLT_49_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
                         + {
                             * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_noteButtonLT_51_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_56 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 400f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 400f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notesLT_49_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
                         + {
                             * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_noteButtonLT_51_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_56 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((600f64 as f64) * (0.75f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((600f64 as f64) * (0.75f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 1f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Start as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_21_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_24_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_24_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_24_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_24_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_21_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_16_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownLT_17_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_42_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_16_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownLT_17_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_42_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
                             + {
                                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
                             + {
                                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_42_layout_cache }
                        ) . apply_pin (_self) . get () [1usize] as _ , r#spacing : 8f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
                         + {
                             * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
                         + {
                             * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
                         + {
                             * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
                         + {
                             * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_42_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Start as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : 600f64 as _ , r#spacing : 20f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_42_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_42_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , 20f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Start as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_register_12_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_register_12_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_registerBox_11_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#End as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_register_12_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((400f64 as f64) * (0.2f64 as f64)) as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ((400f64 as f64) * (0.2f64 as f64)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = ((400f64 as f64) * (0.02f64 as f64)) as _ ;
                             the_struct }
                         as _ , r#size : 600f64 as _ , r#spacing : 8f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_registerBox_11_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_register_12_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((400f64 as f64) * (0.3f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((400f64 as f64) * (0.3f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = ((400f64 as f64) * (0.04f64 as f64)) as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_registerBox_11_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_register_12_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((400f64 as f64) * (0.2f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((400f64 as f64) * (0.2f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = ((400f64 as f64) * (0.02f64 as f64)) as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#End as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_37_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_40_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_40_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_40_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_image_40_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_37_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + (sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#image_40 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownLT_17_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#End as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_buttons_20_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((600f64 as f64) * (0.15f64 as f64)) as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ((600f64 as f64) * (0.15f64 as f64)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : 600f64 as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownLT_17_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_buttons_20_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 400f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 400f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownLT_17_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_buttons_20_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((600f64 as f64) * (0.15f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((600f64 as f64) * (0.15f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#End as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
                             + {
                                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
                             + {
                                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_42_layout_cache }
                        ) . apply_pin (_self) . get () [3usize] as _ , r#spacing : 40f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
                         + {
                             * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
                         + {
                             * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
                         + {
                             * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
                         + {
                             * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 40f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_10_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_10_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_10_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_10_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_14_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_14_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_14_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_14_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_23_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_23_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_23_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_23_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_27_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_27_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_27_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_27_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_31_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_31_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_31_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_31_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_35_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_35_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_35_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_35_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_39_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_39_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_39_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_39_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_54_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_54_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_54_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_54_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_textbox_55_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                     + {
                         * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_textbox_55_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                     + {
                         * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_textbox_55_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                     + {
                         * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_textbox_55_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                     + {
                         * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Slint Window")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (400f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#loginForm_visibility_2 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) . set ({
                 (! false) as bool }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#loginForm_3 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (45f64 as _ , [sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4291456535f64 as u32) , position : 0f64 as _ }
                 , sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4294871583f64 as u32) , position : 1f64 as _ }
                ]))) as slint :: Brush }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((600f64 as f64) * (0.09f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Login")) as sp :: SharedString }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((400f64 as f64) * (0.9f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((400f64 as f64) * (0.04f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
                 + {
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_inputsLT_4_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((600f64 as f64) * (0.09f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Password")) as sp :: SharedString }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((400f64 as f64) * (0.9f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((400f64 as f64) * (0.04f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
                 + {
                     * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_inputsLT_4_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#login_8 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_login_checked }
                    ) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4294901760f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#login_8 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_9 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4290493371f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294703092f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#login_8 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#login_8 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_9 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_login_check }
                            ) . apply_pin (_self) . call (& (sp :: SharedString :: from ("Login") as _ , ({
                                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
                             + {
                                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_9 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_10_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_10_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Login")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_10_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_10_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#register_12 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_13 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (0f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (4291698447f64 as u32) , position : 0f64 as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (4294186241f64 as u32) , position : 1f64 as _ }
                        ])) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294186241f64 as u32))) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#register_12 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_13 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4290493371f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294703092f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#register_12 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#register_12 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_13 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_14_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_14_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("See this app\nfor the first time?")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_14_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_14_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileForm_visibility_15 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) . set ({
                 (! true) as bool }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileForm_16 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (45f64 as _ , [sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4291456535f64 as u32) , position : 0f64 as _ }
                 , sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4294871583f64 as u32) , position : 1f64 as _ }
                ]))) as slint :: Brush }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_visibility_18 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) . set ({
                 (! true) as bool }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_19 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294020658f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_19 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (32f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_19 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (32f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#profile_21 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_selected }
                    ) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4291062553f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294020658f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#profile_21 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_22 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4294703092f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (16513012f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profile_21 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profile_21 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_22 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_select }
                            ) . apply_pin (_self) . call (& () . into ()) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_selected }
                            ) . apply_pin (_self) . set (true as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_selected }
                            ) . apply_pin (_self) . set (false as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_22 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_23_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_23_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_23_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_23_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#image_24 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_24 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_24 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#notes_25 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_selected }
                    ) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4291062553f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294020658f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#notes_25 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_26 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4294703092f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (16513012f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#notes_25 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#notes_25 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_26 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_select }
                            ) . apply_pin (_self) . call (& () . into ()) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_selected }
                            ) . apply_pin (_self) . set (true as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_selected }
                            ) . apply_pin (_self) . set (false as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_26 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_27_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_27_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_27_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_27_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#image_28 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_28 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_28 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#charlist_29 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_selected }
                    ) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4291062553f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294020658f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#charlist_29 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_30 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4294703092f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (16513012f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#charlist_29 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#charlist_29 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_30 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_select }
                            ) . apply_pin (_self) . call (& () . into ()) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_selected }
                            ) . apply_pin (_self) . set (true as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_selected }
                            ) . apply_pin (_self) . set (false as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_30 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_31_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_31_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_31_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_31_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#image_32 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_32 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_32 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#dices_33 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_selected }
                    ) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4291062553f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294020658f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#dices_33 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_34 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4294703092f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (16513012f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#dices_33 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#dices_33 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_34 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_select }
                            ) . apply_pin (_self) . call (& () . into ()) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_selected }
                            ) . apply_pin (_self) . set (true as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_selected }
                            ) . apply_pin (_self) . set (false as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_34 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_35_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_35_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_35_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_35_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#image_36 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_36 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_36 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#settings_37 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_selected }
                    ) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4291062553f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294020658f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#settings_37 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_38 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4294703092f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (16513012f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#settings_37 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#settings_37 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_38 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_select }
                            ) . apply_pin (_self) . call (& () . into ()) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_selected }
                            ) . apply_pin (_self) . set (false as _) ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_selected }
                            ) . apply_pin (_self) . set (true as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_38 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_39_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_39_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_39_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_39_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#image_40 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_40 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_40 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileLT_visibility_41 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) . set ({
                 (! false) as bool }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (32f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Levik")) as sp :: SharedString }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_profileInfo_43_padding = 8f64 ;
                     ;
                     ((((400f64 as f64) - (r#tmp_profileInfo_43_padding . clone () as f64)) as f64) - (r#tmp_profileInfo_43_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (18f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("12345")) as sp :: SharedString }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_profileInfo_43_padding = 8f64 ;
                     ;
                     ((((400f64 as f64) - (r#tmp_profileInfo_43_padding . clone () as f64)) as f64) - (r#tmp_profileInfo_43_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Characters:")) as sp :: SharedString }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_stats_46_padding = 8f64 ;
                     ;
                     ((((400f64 as f64) - (r#tmp_stats_46_padding . clone () as f64)) as f64) - (r#tmp_stats_46_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Adventures:")) as sp :: SharedString }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ({
                     let r#tmp_stats_46_padding = 8f64 ;
                     ;
                     ((((400f64 as f64) - (r#tmp_stats_46_padding . clone () as f64)) as f64) - (r#tmp_stats_46_padding . clone () as f64)) }
                 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notesLT_49_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Notes")) as sp :: SharedString }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (400f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notesLT_49_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#addNote_52 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_53 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (0f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (4291698447f64 as u32) , position : 0f64 as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (4294186241f64 as u32) , position : 1f64 as _ }
                        ])) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294186241f64 as u32))) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#addNote_52 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_53 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4290493371f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294703092f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#addNote_52 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (360f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#addNote_52 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_53 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_note_list }
                            ) . apply_pin (_self) . get ()) . set_row_data (((match & ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_note_list }
                            ) . apply_pin (_self) . get () {
                                 x => {
                                     x . model_tracker () . track_row_count_changes () ;
                                     x . row_count () as i32 }
                                 }
                             as f64) + (1f64 as f64)) as usize , 1f64 as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_53 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_54_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_54_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_54_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_text_54_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_textbox_55_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_textbox_55_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("+")) as sp :: SharedString }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_textbox_55_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_textbox_55_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((400f64 as f64) * (0.1f64 as f64)) as f64) - (({
                         InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                     + {
                         * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((400f64 as f64) * (0.1f64 as f64)) as f64) - (({
                         InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                     + {
                         * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_56 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_56 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (900f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_56 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (400f64 as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_29_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_33_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_flickable_56_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_3_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_3_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_visibility_2_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_visibility_2_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_visibility_2_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_visibility_2_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_noteButtonLT_51_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_25_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_21_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_16_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_16_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_visibility_15_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_visibility_15_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_visibility_15_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_visibility_15_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_visibility_41_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_visibility_41_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_visibility_41_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_visibility_41_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_37_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_19_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_visibility_18_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_visibility_18_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_visibility_18_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_visibility_18_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_13_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_13_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_22_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_22_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_26_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_26_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_30_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_30_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_34_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_34_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_38_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_38_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_53_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_53_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_9_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_9_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#loginForm_visibility_2 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#loginForm_visibility_2 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#loginForm_visibility_2 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#loginForm_visibility_2 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#loginForm_visibility_2 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#loginForm_visibility_2 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#loginForm_3 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_touchArea_66_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_touchArea_66_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_text_visibility_62_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_touchArea_66_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_touchArea_66_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#login_8 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#login_8 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_9 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_9 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#register_12 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#register_12 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_13 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_13 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileForm_visibility_15 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileForm_visibility_15 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileForm_visibility_15 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileForm_visibility_15 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileForm_visibility_15 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileForm_visibility_15 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileForm_16 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_visibility_18 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_visibility_18 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_visibility_18 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_visibility_18 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_visibility_18 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_visibility_18 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_19 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_19 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_19 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_19 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_19 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_19 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_19 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_19 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profile_21 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profile_21 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_22 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_22 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_24 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_24 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_24 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_24 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_24 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#notes_25 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#notes_25 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_26 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_26 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_28 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_28 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_28 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_28 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_28 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#charlist_29 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#charlist_29 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_30 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_30 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_32 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_32 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_32 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_32 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_32 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#dices_33 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#dices_33 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_34 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_34 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_36 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_36 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_36 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_36 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_36 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#settings_37 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#settings_37 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_38 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_38 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_40 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_40 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_40 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_40 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_40 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileLT_visibility_41 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileLT_visibility_41 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileLT_visibility_41 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileLT_visibility_41 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileLT_visibility_41 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileLT_visibility_41 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#addNote_52 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#addNote_52 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_53 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_53 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_56 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             Innerinput_root_60 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#username_5 }
             . apply_pin (x)) ,) ;
             Innerinput_root_60 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#password_6 }
             . apply_pin (x)) ,) ;
             InnertextBox_root_67 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#nickname_44 }
             . apply_pin (x)) ,) ;
             InnertextBox_root_67 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#id_45 }
             . apply_pin (x)) ,) ;
             InnertextBox_root_67 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#Characters_47 }
             . apply_pin (x)) ,) ;
             InnertextBox_root_67 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#adventures_48 }
             . apply_pin (x)) ,) ;
             InnertextBox_root_67 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#textbox_50 }
             . apply_pin (x)) ,) ;
             InnertextBox_root_67 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#textbox_55 }
             . apply_pin (x)) ,) ;
             (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . renderer () . register_font_from_memory (SLINT_EMBEDDED_RESOURCE_5 . into ()) . unwrap () ;
             (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . renderer () . register_font_from_memory (SLINT_EMBEDDED_RESOURCE_6 . into ()) . unwrap () ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_textbox_58 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 400f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 400f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 600f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 600f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_textbox_58 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_textbox_58 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (600f64 as sp :: Coord , 400f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_visibility_2_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_visibility_2_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_visibility_2_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_visibility_2_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_visibility_15_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_visibility_15_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_visibility_15_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_visibility_15_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (600f64 as sp :: Coord , 400f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 => (600f64 as sp :: Coord , 400f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_3_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginForm_3_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 => (((600f64 as f64) * (0.09f64 as f64)) as sp :: Coord , ((400f64 as f64) * (0.9f64 as f64)) as sp :: Coord , ((400f64 as f64) * (0.04f64 as f64)) as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_inputsLT_4_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 6u32 => (((600f64 as f64) * (0.09f64 as f64)) as sp :: Coord , ((400f64 as f64) * (0.9f64 as f64)) as sp :: Coord , ((400f64 as f64) * (0.04f64 as f64)) as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_inputsLT_4_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 7u32 => (85.71428571428571f64 as sp :: Coord , {
                     let r#tmp_loginLT_7_padding = ((- 400f64 as f64) * (((- 20f64 as f64) * (0.01f64 as f64)) as f64)) ;
                     ;
                     ((((400f64 as f64) - (r#tmp_loginLT_7_padding . clone () as f64)) as f64) - (r#tmp_loginLT_7_padding . clone () as f64)) }
                 as sp :: Coord , ((- 400f64 as f64) * (((- 20f64 as f64) * (0.01f64 as f64)) as f64)) as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_loginLT_7_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 8u32 => (((400f64 as f64) * (0.2f64 as f64)) as sp :: Coord , ((400f64 as f64) * (0.3f64 as f64)) as sp :: Coord , ((400f64 as f64) * (0.04f64 as f64)) as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_registerBox_11_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 17u32 => (85.71428571428571f64 as sp :: Coord , {
                     let r#tmp_loginLT_7_padding = ((- 400f64 as f64) * (((- 20f64 as f64) * (0.01f64 as f64)) as f64)) ;
                     ;
                     ((((400f64 as f64) - (r#tmp_loginLT_7_padding . clone () as f64)) as f64) - (r#tmp_loginLT_7_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_9_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_9_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 18u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , (((({
                     let r#tmp_loginLT_7_padding = ((- 400f64 as f64) * (((- 20f64 as f64) * (0.01f64 as f64)) as f64)) ;
                     ;
                     ((((400f64 as f64) - (r#tmp_loginLT_7_padding . clone () as f64)) as f64) - (r#tmp_loginLT_7_padding . clone () as f64)) }
                 as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((85.71428571428571f64 as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 19u32 => (((400f64 as f64) * (0.2f64 as f64)) as sp :: Coord , ((400f64 as f64) * (0.3f64 as f64)) as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_13_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_13_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 20u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((((400f64 as f64) * (0.3f64 as f64)) as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((((400f64 as f64) * (0.2f64 as f64)) as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 21u32 => (600f64 as sp :: Coord , 400f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_16_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileForm_16_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 22u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_visibility_18_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_visibility_18_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_visibility_18_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_visibility_18_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 23u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_visibility_41_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_visibility_41_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_visibility_41_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_visibility_41_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 24u32 => (((600f64 as f64) * (0.15f64 as f64)) as sp :: Coord , 400f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_19_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_sidedownLT_17_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 25u32 => (((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as sp :: Coord , 400f64 as sp :: Coord , 0f64 as sp :: Coord , ((((((600f64 as f64) * (0.15f64 as f64)) as f64) - (((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 26u32 => (((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as sp :: Coord , 57.142857142857146f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_buttons_20_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_21_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 27u32 => (((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as sp :: Coord , 57.142857142857146f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_buttons_20_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_25_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 28u32 => (((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as sp :: Coord , 57.142857142857146f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_buttons_20_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_29_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 29u32 => (((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as sp :: Coord , 57.142857142857146f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_buttons_20_layout_cache }
                ) . apply_pin (_self) . get () [6usize] as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_33_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 30u32 => (((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as sp :: Coord , 57.142857142857146f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_buttons_20_layout_cache }
                ) . apply_pin (_self) . get () [8usize] as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_37_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 31u32 => (((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as sp :: Coord , 57.142857142857146f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_22_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_22_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 32u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((57.142857142857146f64 as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 33u32 => ((({
                     let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                     ;
                     (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                 as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord , ((57.142857142857146f64 as f64) * (0.9f64 as f64)) as sp :: Coord , ((((57.142857142857146f64 as f64) - (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as f64) - ((({
                     let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                     ;
                     (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                 as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 34u32 => (((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as sp :: Coord , 57.142857142857146f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_26_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_26_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 35u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((57.142857142857146f64 as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 36u32 => ((({
                     let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                     ;
                     (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                 as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord , ((57.142857142857146f64 as f64) * (0.9f64 as f64)) as sp :: Coord , ((((57.142857142857146f64 as f64) - (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as f64) - ((({
                     let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                     ;
                     (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                 as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 37u32 => (((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as sp :: Coord , 57.142857142857146f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_30_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_30_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 38u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((57.142857142857146f64 as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 39u32 => ((({
                     let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                     ;
                     (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                 as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord , ((57.142857142857146f64 as f64) * (0.9f64 as f64)) as sp :: Coord , ((((57.142857142857146f64 as f64) - (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as f64) - ((({
                     let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                     ;
                     (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                 as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 40u32 => (((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as sp :: Coord , 57.142857142857146f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_34_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_34_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 41u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((57.142857142857146f64 as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 42u32 => ((({
                     let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                     ;
                     (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                 as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord , ((57.142857142857146f64 as f64) * (0.9f64 as f64)) as sp :: Coord , ((((57.142857142857146f64 as f64) - (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as f64) - ((({
                     let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                     ;
                     (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                 as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 43u32 => (((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as sp :: Coord , 57.142857142857146f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_38_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_38_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 44u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((57.142857142857146f64 as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 45u32 => ((({
                     let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                     ;
                     (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                 as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord , ((57.142857142857146f64 as f64) * (0.9f64 as f64)) as sp :: Coord , ((((57.142857142857146f64 as f64) - (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((((((600f64 as f64) * (0.15f64 as f64)) as f64) * (0.5f64 as f64)) as f64) - ((({
                     let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                     ;
                     (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                 as f64) * (((57.142857142857146f64 as f64) * (0.9f64 as f64)) as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 46u32 => (600f64 as sp :: Coord , 400f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 47u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_42_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , 400f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_42_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 48u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_42_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , 400f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileLT_42_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 49u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_profileInfo_43_padding = 8f64 ;
                     ;
                     ((((400f64 as f64) - (r#tmp_profileInfo_43_padding . clone () as f64)) as f64) - (r#tmp_profileInfo_43_padding . clone () as f64)) }
                 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 50u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_profileInfo_43_padding = 8f64 ;
                     ;
                     ((((400f64 as f64) - (r#tmp_profileInfo_43_padding . clone () as f64)) as f64) - (r#tmp_profileInfo_43_padding . clone () as f64)) }
                 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profileInfo_43_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 51u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_stats_46_padding = 8f64 ;
                     ;
                     ((((400f64 as f64) - (r#tmp_stats_46_padding . clone () as f64)) as f64) - (r#tmp_stats_46_padding . clone () as f64)) }
                 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 52u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_stats_46_padding = 8f64 ;
                     ;
                     ((((400f64 as f64) - (r#tmp_stats_46_padding . clone () as f64)) as f64) - (r#tmp_stats_46_padding . clone () as f64)) }
                 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_stats_46_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 53u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notesLT_49_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , 400f64 as sp :: Coord , ({
                     InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notesLT_49_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 54u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notesLT_49_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , 400f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_noteButtonLT_51_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notesLT_49_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 55u32 => (((600f64 as f64) * (0.75f64 as f64)) as sp :: Coord , 400f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_flickable_56_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notesLT_49_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord ,) , 56u32 => (((400f64 as f64) * (0.1f64 as f64)) as sp :: Coord , ((400f64 as f64) * (0.1f64 as f64)) as sp :: Coord , ((400f64 as f64) * (0.9f64 as f64)) as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_noteButtonLT_51_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 57u32 => (((400f64 as f64) * (0.1f64 as f64)) as sp :: Coord , ((400f64 as f64) * (0.1f64 as f64)) as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_53_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_touchArea_53_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 58u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((((400f64 as f64) * (0.1f64 as f64)) as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((((400f64 as f64) * (0.1f64 as f64)) as f64) - (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 59u32 => (({
                     InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((((400f64 as f64) * (0.1f64 as f64)) as f64) - (({
                     InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((((400f64 as f64) * (0.1f64 as f64)) as f64) - (({
                     InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 60u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_56 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_56 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_56 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_56 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 9u32 ..= 12u32 => return {
                     * & Self :: FIELD_OFFSETS . r#username_5 }
                 . apply_pin (_self) . item_geometry (index - 9u32 + 1) , 13u32 ..= 16u32 => return {
                     * & Self :: FIELD_OFFSETS . r#password_6 }
                 . apply_pin (_self) . item_geometry (index - 13u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 18u32 => sp :: r#AccessibleRole :: r#Text , 20u32 => sp :: r#AccessibleRole :: r#Text , 32u32 => sp :: r#AccessibleRole :: r#Text , 35u32 => sp :: r#AccessibleRole :: r#Text , 38u32 => sp :: r#AccessibleRole :: r#Text , 41u32 => sp :: r#AccessibleRole :: r#Text , 44u32 => sp :: r#AccessibleRole :: r#Text , 49u32 => sp :: r#AccessibleRole :: r#Text , 50u32 => sp :: r#AccessibleRole :: r#Text , 51u32 => sp :: r#AccessibleRole :: r#Text , 52u32 => sp :: r#AccessibleRole :: r#Text , 53u32 => sp :: r#AccessibleRole :: r#Text , 58u32 => sp :: r#AccessibleRole :: r#Text , 59u32 => sp :: r#AccessibleRole :: r#Text , 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#username_5 }
                 . apply_pin (_self) . accessible_role (0) , 9u32 ..= 12u32 => {
                     * & Self :: FIELD_OFFSETS . r#username_5 }
                 . apply_pin (_self) . accessible_role (index - 9u32 + 1) , 6u32 => {
                     * & Self :: FIELD_OFFSETS . r#password_6 }
                 . apply_pin (_self) . accessible_role (0) , 13u32 ..= 16u32 => {
                     * & Self :: FIELD_OFFSETS . r#password_6 }
                 . apply_pin (_self) . accessible_role (index - 13u32 + 1) , 49u32 => {
                     * & Self :: FIELD_OFFSETS . r#nickname_44 }
                 . apply_pin (_self) . accessible_role (0) , 50u32 => {
                     * & Self :: FIELD_OFFSETS . r#id_45 }
                 . apply_pin (_self) . accessible_role (0) , 51u32 => {
                     * & Self :: FIELD_OFFSETS . r#Characters_47 }
                 . apply_pin (_self) . accessible_role (0) , 52u32 => {
                     * & Self :: FIELD_OFFSETS . r#adventures_48 }
                 . apply_pin (_self) . accessible_role (0) , 53u32 => {
                     * & Self :: FIELD_OFFSETS . r#textbox_50 }
                 . apply_pin (_self) . accessible_role (0) , 59u32 => {
                     * & Self :: FIELD_OFFSETS . r#textbox_55 }
                 . apply_pin (_self) . accessible_role (0) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (18u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("Login") , (20u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("See this app\nfor the first time?") , (32u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("") , (35u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("") , (38u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("") , (41u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("") , (44u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("") , (49u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (50u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (51u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (52u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (53u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (58u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("") , (59u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#username_5 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (9u32 ..= 12u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#username_5 }
                 . apply_pin (_self) . accessible_string_property (index - 9u32 + 1 , what) , (6u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#password_6 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (13u32 ..= 16u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#password_6 }
                 . apply_pin (_self) . accessible_string_property (index - 13u32 + 1 , what) , (49u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#nickname_44 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (50u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#id_45 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (51u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#Characters_47 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (52u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#adventures_48 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (53u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#textbox_50 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (59u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#textbox_55 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_textbox_58 {
         r#textbox_58 : InnertextBox_root_67 , r#model_data : sp :: Property < i32 > , r#model_index : sp :: Property < i32 > , r#textbox_58_min_height : sp :: Property < sp :: LogicalLength > , r#textbox_58_min_width : sp :: Property < sp :: LogicalLength > , r#textbox_58_preferred_height : sp :: Property < sp :: LogicalLength > , r#textbox_58_preferred_width : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_textbox_58 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_textbox_58 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnertextBox_root_67 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#textbox_58 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
                     + {
                         * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
                     + {
                         * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
                     + {
                         * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
                     + {
                         * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("dawwdas")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerComponent_textbox_58 :: FIELD_OFFSETS . r#model_index }
                    ) . apply_pin (_self) . get () as f64) * (20f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67_x }
            ) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnertextBox_root_67 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#textbox_58 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             ({
                 * & InnerComponent_textbox_58 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . get () as usize }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 10f64 as sp :: Coord , ((({
                     * & InnerComponent_textbox_58 :: FIELD_OFFSETS . r#model_index }
                ) . apply_pin (_self) . get () as f64) * (20f64 as f64)) as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Text , 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#textbox_58 }
                 . apply_pin (_self) . accessible_role (0) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
                 + {
                     * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#textbox_58 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent_textbox_58 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_textbox_58 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 InnerComponent_textbox_58 :: FIELD_OFFSETS . r#textbox_58 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_textbox_58) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_textbox_58 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_textbox_58 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_textbox_58 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_textbox_58 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 61u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_textbox_58 {
         type Data = i32 ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             ({
                 * & InnerComponent_textbox_58 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . set (_index as _) ;
             ({
                 * & InnerComponent_textbox_58 :: FIELD_OFFSETS . r#model_data }
            ) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_textbox_58 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerMainWindow {
         pub fn new () -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & self_rc) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & self_rc , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             62usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 21u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 53u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 5u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 9u32 , parent_index : 4u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 13u32 , parent_index : 4u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 17u32 , parent_index : 4u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 19u32 , parent_index : 4u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 12u32 , parent_index : 5u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 13u32 , parent_index : 5u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 13u32 , parent_index : 5u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 13u32 , parent_index : 9u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 16u32 , parent_index : 6u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 17u32 , parent_index : 6u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 17u32 , parent_index : 6u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 17u32 , parent_index : 13u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 19u32 , parent_index : 7u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 19u32 , parent_index : 7u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 21u32 , parent_index : 8u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 21u32 , parent_index : 8u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 22u32 , parent_index : 2u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 24u32 , parent_index : 21u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 46u32 , parent_index : 21u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 25u32 , parent_index : 22u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 5u32 , children_index : 26u32 , parent_index : 24u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 31u32 , parent_index : 25u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 34u32 , parent_index : 25u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 37u32 , parent_index : 25u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 40u32 , parent_index : 25u32 , item_array_index : 29u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 43u32 , parent_index : 25u32 , item_array_index : 30u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 34u32 , parent_index : 26u32 , item_array_index : 31u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 34u32 , parent_index : 26u32 , item_array_index : 32u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 34u32 , parent_index : 26u32 , item_array_index : 33u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 37u32 , parent_index : 27u32 , item_array_index : 34u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 37u32 , parent_index : 27u32 , item_array_index : 35u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 37u32 , parent_index : 27u32 , item_array_index : 36u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 40u32 , parent_index : 28u32 , item_array_index : 37u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 40u32 , parent_index : 28u32 , item_array_index : 38u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 40u32 , parent_index : 28u32 , item_array_index : 39u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 43u32 , parent_index : 29u32 , item_array_index : 40u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 43u32 , parent_index : 29u32 , item_array_index : 41u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 43u32 , parent_index : 29u32 , item_array_index : 42u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 46u32 , parent_index : 30u32 , item_array_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 46u32 , parent_index : 30u32 , item_array_index : 44u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 46u32 , parent_index : 30u32 , item_array_index : 45u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 47u32 , parent_index : 23u32 , item_array_index : 46u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 49u32 , parent_index : 46u32 , item_array_index : 47u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 51u32 , parent_index : 46u32 , item_array_index : 48u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 51u32 , parent_index : 47u32 , item_array_index : 49u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 51u32 , parent_index : 47u32 , item_array_index : 50u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 53u32 , parent_index : 48u32 , item_array_index : 51u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 53u32 , parent_index : 48u32 , item_array_index : 52u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 56u32 , parent_index : 3u32 , item_array_index : 53u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 56u32 , parent_index : 3u32 , item_array_index : 54u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 60u32 , parent_index : 3u32 , item_array_index : 55u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 57u32 , parent_index : 54u32 , item_array_index : 56u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 60u32 , parent_index : 56u32 , item_array_index : 57u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 60u32 , parent_index : 56u32 , item_array_index : 58u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 60u32 , parent_index : 56u32 , item_array_index : 59u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 61u32 , parent_index : 55u32 , item_array_index : 60u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 60u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerMainWindow , sp :: ItemVTable , sp :: AllowPin > ;
             61usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#loginForm_visibility_2 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileForm_visibility_15 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#notesLT_49 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#loginForm_3 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#root_60 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#login_8 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#register_12 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_visibility_62 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#touchArea_66 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#username_5 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_visibility_62 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#textInput_65 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#touchArea_66 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#password_6 }
             + {
                 * & Innerinput_root_60 :: FIELD_OFFSETS . r#text_63 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_9 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_13 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_14 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileForm_16 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_visibility_18 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileLT_visibility_41 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#sidedownMenu_19 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#buttons_20 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profile_21 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#notes_25 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#charlist_29 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#dices_33 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#settings_37 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_22 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_23 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_24 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_26 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_27 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_28 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_30 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_31 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_32 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_34 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_35 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_36 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_38 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_39 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#image_40 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileLT_42 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#profileInfo_43 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#stats_46 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#nickname_44 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#id_45 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#Characters_47 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#adventures_48 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_50 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#noteButtonLT_51 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_56 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#addNote_52 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touchArea_53 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_54 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#textbox_55 }
             + {
                 * & InnertextBox_root_67 :: FIELD_OFFSETS . r#root_67 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_viewport_57 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             sp :: Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self ,) -> sp :: Result < & sp :: Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter_ . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let self_rc = sp :: VRcMapped :: origin (& self . self_weak . get () . unwrap () . upgrade () . unwrap () ,) ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& self_rc) ;
                 core :: result :: Result :: Ok (adapter) }
            ) }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter_ . get () . cloned () }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerMainWindow) ;
         }
     ;
     impl sp :: PinnedDrop for InnerMainWindow {
         fn drop (self : core :: pin :: Pin < & mut InnerMainWindow >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerMainWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerMainWindow > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#MainWindow (sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow >) ;
     impl r#MainWindow {
         pub fn new () -> core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerMainWindow :: new () ? ;
             inner . globals . global_ColorSchemeSelector_68 . clone () . init (& inner) ;
             inner . globals . global_FluentPalette_69 . clone () . init (& inner) ;
             inner . globals . global_StyleMetrics_70 . clone () . init (& inner) ;
             InnerMainWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn invoke_addNote (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_addNote }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_addNote (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_addNote }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_charlist_select (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_select }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_charlist_select (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_select }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn get_charlist_selected (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_selected }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_charlist_selected (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_charlist_selected }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_dices_select (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_select }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_dices_select (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_select }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn get_dices_selected (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_selected }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_dices_selected (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_dices_selected }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_login_check (& self , arg_0 : sp :: SharedString , arg_1 : sp :: SharedString ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_login_check }
            ) . apply_pin (_self) . call (& (arg_0 , arg_1 ,)) }
         # [allow (dead_code)] pub fn on_login_check (& self , mut f : impl FnMut (sp :: SharedString , sp :: SharedString) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_login_check }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone () , args . 1 . clone ())) }
         # [allow (dead_code)] pub fn get_login_checked (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_login_checked }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_login_checked (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_login_checked }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_name (& self) -> i32 {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_name }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_name (& self , value : i32) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_name }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_note (& self) -> sp :: ModelRc < (sp :: SharedString , sp :: SharedString , i32 ,) > {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_note }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_note (& self , value : sp :: ModelRc < (sp :: SharedString , sp :: SharedString , i32 ,) >) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_note }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_notes_ammount (& self) -> i32 {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_ammount }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_notes_ammount (& self , value : i32) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_ammount }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_notes_select (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_select }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_notes_select (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_select }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn get_notes_selected (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_selected }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_notes_selected (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_notes_selected }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_profile_select (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_select }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_profile_select (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_select }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn get_profile_selected (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_selected }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_profile_selected (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_profile_selected }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_settings_select (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_select }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_settings_select (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_select }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn get_settings_selected (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_selected }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_settings_selected (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_settings_selected }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] fn get_note_list (& self , _private_property : ()) {
             }
         # [allow (dead_code)] fn set_note_list (& self , _private_property : ()) {
             }
         }
     impl From < r#MainWindow > for sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow > {
         fn from (value : r#MainWindow) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#MainWindow {
         type Inner = InnerMainWindow ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (& self . 0) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn from_inner (inner : sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow >) -> Self {
             Self (inner) }
         fn run (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             slint :: run_event_loop () ? ;
             self . hide () ? ;
             core :: result :: Result :: Ok (()) }
         fn show (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     # [allow (dead_code)] struct Globals_MainWindow {
         global_ColorSchemeSelector_68 : :: core :: pin :: Pin < sp :: Rc < InnerColorSchemeSelector_68 >> , global_FluentPalette_69 : :: core :: pin :: Pin < sp :: Rc < InnerFluentPalette_69 >> , global_StyleMetrics_70 : :: core :: pin :: Pin < sp :: Rc < InnerStyleMetrics_70 >> , }
     impl :: core :: default :: Default for Globals_MainWindow {
         fn default () -> Self {
             Self {
                 global_ColorSchemeSelector_68 : InnerColorSchemeSelector_68 :: new () , global_FluentPalette_69 : InnerFluentPalette_69 :: new () , global_StyleMetrics_70 : InnerStyleMetrics_70 :: new () , }
             }
         }
     static SLINT_EMBEDDED_RESOURCE_5 : & 'static [u8] = :: core :: include_bytes ! ("/Users/levik/Desktop/rust/DnDHelper/ui/fonts/MetalMania-Regular.ttf") ;
     static SLINT_EMBEDDED_RESOURCE_4 : & 'static [u8] = :: core :: include_bytes ! ("/Users/levik/Desktop/rust/DnDHelper/ui/resources/setting-2.svg") ;
     static SLINT_EMBEDDED_RESOURCE_2 : & 'static [u8] = :: core :: include_bytes ! ("/Users/levik/Desktop/rust/DnDHelper/ui/resources/charlist.svg") ;
     static SLINT_EMBEDDED_RESOURCE_0 : & 'static [u8] = :: core :: include_bytes ! ("/Users/levik/Desktop/rust/DnDHelper/ui/resources/user-edit.svg") ;
     static SLINT_EMBEDDED_RESOURCE_6 : & 'static [u8] = :: core :: include_bytes ! ("/Users/levik/Desktop/rust/DnDHelper/ui/fonts/zed-mono-extended.ttf") ;
     static SLINT_EMBEDDED_RESOURCE_1 : & 'static [u8] = :: core :: include_bytes ! ("/Users/levik/Desktop/rust/DnDHelper/ui/resources/book.svg") ;
     static SLINT_EMBEDDED_RESOURCE_3 : & 'static [u8] = :: core :: include_bytes ! ("/Users/levik/Desktop/rust/DnDHelper/ui/resources/dices.svg") ;
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_5_1 = slint :: VersionCheck_1_5_1 ;
     }
 # [allow (unused_imports)] pub use slint_generatedMainWindow :: {
     r#MainWindow }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
