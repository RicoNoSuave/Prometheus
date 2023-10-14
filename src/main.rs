use dotenv::dotenv;
use eframe::{
    App,
    CreationContext,
    egui::{
        self,
        Align,
        Button,
        CentralPanel,
        Color32,
        ComboBox,
        Context,
        FontData,
        FontDefinitions,
        FontFamily::{
            self,
            Proportional
        },
        FontId,
        Hyperlink,
        Id,
        Image,
        include_image,
        Key,
        Label,
        Layout,
        menu,
        Painter,
        Response,
        RichText,
        ScrollArea,
        Sense,
        Separator,
        TextEdit,
        TopBottomPanel,
        Ui,
        Vec2,
        vec2,
        Visuals
    },
    epaint::{Rgba, Rect},
    Frame,
    IconData,
    NativeOptions,
    run_native, emath::Align2
};
use egui_extras::install_image_loaders;
use image::ImageError;
use std::{
    fs::read,
    io::Error,
    path::Path
};
use newsapi::{
    Category,
    Country,
    NewsAPIError,
    NewsAPIResponse,
    NewsCard
};

const PADDING: f32 = 5.;

enum Color {
    Cyan,
    White
}

impl Color {
    fn color(&self) -> Color32 {
        match self {
            Self::Cyan => Color32::from_rgb(0, 255, 255),
            Self::White => Color32::from_rgb(0,0,0)
        }
    }
}

// Enum for text size
enum TextSize {
    Large,
    Medium,
    Small
}

// Enum for adding textsize to text
enum TextStyle {
    Heading,
    StaticButton,
    Search,
    Title,
    Body,
    Button
}

impl TextStyle {
    fn set_style(&self, size: &TextSize) -> FontId {
        let modifier: f32;
        match size {
            TextSize::Large => modifier = 4.,
            TextSize::Medium => modifier = 2.,
            TextSize::Small => modifier = 0.
        };

		match self {
			Self::Heading => FontId::new(30.0, Proportional),
            Self::Search => FontId::new(16.0 + modifier, Proportional),
            Self::StaticButton => FontId::new(20.0, Proportional),
            Self::Title => FontId::new(14.0 + modifier, Proportional),
			Self::Body => FontId::new(12.0 + modifier, Proportional),
			Self::Button => FontId::new(10.0 + modifier, Proportional),
		}
    }
}

// Prometheus structure
struct Interface{
    article: Option<NewsCard>,
    category: Category,
    country: Country,
    display_country: bool,
    display_search: bool,
    display_settings: bool,
    news: Result<Vec<NewsCard>, NewsAPIError>,
    search: String,
    search_topic: Option<String>,
    startup: bool,
    text_size: TextSize,
}

impl Interface {
    fn new() -> Self {
        // Get api key
        dotenv().unwrap();
        let api_key: String = std::env::var("API_KEY").unwrap();

        // Generate self
        Self {
            article: None,
            category: Category::General,
            country: Country::UnitedStates,
            display_country: false,
            display_search: false,
            display_settings: false,
            news: NewsAPIResponse::new(
                    api_key,
                    &Category::General,
                    &Country::UnitedStates,
                    ""
                ),
            search: "".to_string(),
            search_topic: None,
            startup: true,
            text_size: TextSize::Small,
        }
    }

    // helper to set fonts
    fn configure_fonts(&mut self, ctx: &Context) {
        let mut font_def: FontDefinitions = FontDefinitions::default();

        // Install MesloLGS
        font_def
            .font_data
            .insert(
                "MesloLGS".to_owned(),
                FontData::from_static(
                    include_bytes!("../fonts/MesloLGS_NF_Regular.ttf")
                )
            );

        // Set MesloLGS to highest priority
        font_def
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_owned());

        // Set MesloLGS as fallback
        font_def
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("MesloLGS".to_owned());

        ctx.set_fonts(font_def);
    }

    // update function to update news
    fn internal_update(&mut self) {
        // Get api key
        dotenv().unwrap();
        let api_key: String = std::env::var("API_KEY").unwrap();

        // Resubmit for update
        self.news = NewsAPIResponse::new(
            api_key,
            &self.category,
            &self.country,
            &self.search
        );

        self.article = None;
    }

    // Render UI controller (allows for display of errors)
    fn render_ui(&mut self, ui: &mut Ui) {
        if self.article.is_some() {
            self.render_article(ui);
        } else {
            match &self.news {
                Ok(_e) => self.render_news(ui),
                Err(e) =>{
                    ui.add(Label::new(format!("{}", e.to_string())));
                }
            }
        }
    }

    fn render_article(&mut self, ui: &mut Ui) {
        let news: NewsCard = self.article.clone().unwrap();
        ui.add_space(PADDING);

        // Create fields
        let mut trim_title: &str = "";
        let split_title = news.title().split(" - ");
        for str in split_title {
            if trim_title == "" {
                trim_title = str;
            }
        }

        let title: RichText = enrich(
            trim_title,
            &TextStyle::Title,
            &self.text_size
        );
        let date: RichText = enrich(
            &news.date(),
            &TextStyle::Body,
            &self.text_size
        );
        let mut author: Option<RichText> = None;
        if news.author().is_some() {
            author = Some(enrich(
                &format!("By: {}", news.author().unwrap()),
                &TextStyle::Body,
                    &self.text_size
                )
            )
        }
        let mut trim_content: &str = "";
        let split_content = news.content().unwrap().split("[");
        for str in split_content {
            if trim_content == "" {
                trim_content = str;
            }
        }
        let content: RichText = enrich(
            trim_content,
            &TextStyle::Body,
            &self.text_size
        );

        // Set Color
        let title_color: Color = Color::White;

        // Render
        ui.colored_label(title_color.color(), title);
        ui.add_space(PADDING);
        if author.is_some() {
            ui.add(Label::new(author.unwrap()));
            ui.add_space(PADDING);
        }
        ui.add(Label::new(date));
        ui.add_space(PADDING);
        ui.add(Label::new(content));
        ui.add(Label::new(""));

        // Add below links and buttons
        let ret_btn_txt: RichText = enrich(
            "return to news",
            &TextStyle::Button,
            &self.text_size
        );

        let url_txt = enrich(
            "read more online ⤴",
            &TextStyle::Button,
            &self.text_size
        );

        menu::bar(ui, |ui| 
            {
                ui.with_layout(
                    Layout::left_to_right(Align::Min),
                    |ui: &mut Ui| {
                        ui.add_space(PADDING);
                        let ret_btn = ui.add(Button::new(ret_btn_txt));
                        if ret_btn.clicked() {
                            self.article = None;
                        }
                    }
                );

                let url_color: Color = Color::Cyan;
                ui.style_mut()
                    .visuals.hyperlink_color = url_color.color();

                ui.with_layout(
                    Layout::right_to_left(Align::Min),
                    |ui| {
                        ui.add_space(PADDING);
                        ui.add(Hyperlink::from_label_and_url(url_txt, news.url()));
                    }
                );
            }
        );
    }

    // Render news as presented
    fn render_news(&mut self, ui: &mut Ui) {
        // Create iter over news
        let mut iter: std::slice::Iter<'_, NewsCard> = 
            self
            .news
            .as_ref()
            .unwrap()
            .iter();

        // While there are news articles in news
        while let Some(newscard) = iter.next() {
            // If the news article wasn't removed at call
            if newscard.title() != "[Removed]" {
                // Pad
                ui.add_space(PADDING);

                // Create title
                let title: RichText = enrich(
                    &format!("▶ {}", newscard.title()),
                    &TextStyle::Title,
                    &self.text_size
                );

                // Set Color
                let title_color: Color = Color::White;

                // Render
                ui.colored_label(title_color.color(), title);

                // Create description if it exists
                if newscard.description().is_some() {
                    // Pad
                    ui.add_space(PADDING);

                    // Create Object
                    let description: RichText = enrich(
                        newscard.description().unwrap(),
                        &TextStyle::Body,
                        &self.text_size
                    );

                    // Render
                    ui.add(Label::new(description));
                }

                egui::menu::bar(ui, |ui| 
                    {
                        if newscard.content().is_some() {
                            ui.with_layout(Layout::left_to_right(Align::Max),
                                |ui| {
                                    let ret_btn_txt: RichText = enrich(
                                        "read article",
                                        &TextStyle::Button,
                                        &self.text_size
                                    );
        
                                    ui.add_space(PADDING);
                                    let prev = ui.add(Button::new(ret_btn_txt));
                                    if prev.clicked() {
                                        self.article = Some(newscard.clone());
                                    }
                                }
                            );
                        }
        
                        let url_color: Color = Color::Cyan;
                        ui.style_mut()
                            .visuals.hyperlink_color = url_color.color();
        
                        ui.with_layout(
                            Layout::right_to_left(Align::Max),
                            |ui| {
                                ui.add_space(PADDING);
                                // Create Link
                                let link: RichText = enrich(
                                    "read more online ⤴",
                                    &TextStyle::Button,
                                    &self.text_size);

                                ui.add(Hyperlink::from_label_and_url(link, newscard.url()));
                            }
                        );
                    }
                );

                ui.add(Separator::default());
            }
        }
    }

    // Header function
    fn render_header(&mut self, ui: &mut Ui) {

        // Set header to single row
        egui::menu::bar(ui, |ui| {
            // Start from left
            ui.with_layout(Layout::left_to_right(Align::Center),
                |ui| {
                    // Side buffer
                    ui.add_space(PADDING);

                    // Get current category
                    let heading: RichText = enrich(
                        &self.category.to_string(),
                        &TextStyle::Heading,
                        &self.text_size
                    );

                    // Create combo box of category
                    ComboBox::from_label(" ")
                        .selected_text(heading)
                        .width(180.)
                        .show_ui(ui,
                            |ui: &mut Ui| {
                                // Iterate
                                for i in category_vec() {
                                    // Get response
                                    if ui
                                        .selectable_value(
                                            &mut self.category,
                                            i,
                                            i.to_string()
                                        )
                                        .clicked() {
                                            self.display_search = false;
                                            self.search = "".to_string();
                                            self.internal_update();
                                        }
                                }
                            }
                        );
                    }
                );

            // Now do right side, shifted down and right to bottom
            ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                // Side padding
                ui.add_space(PADDING);

                // Create settings response
                let settings_btn: Button<'_> = header_button("🔧");
                let settings: Response = ui.add(settings_btn);

                // Handle settings
                if settings.clicked() && !self.display_settings {
                    self.display_settings = true;
                }

                // Add buttons if settings clicked
                if self.display_settings {
                    // Create country button
                    let cntry_btn: Button<'_> = header_button("🚩");
                    let country: Response = ui.add(cntry_btn);

                    // Handle country response
                    if country.clicked() {
                        self.display_country = true;
                    }

                    // Create refresh button
                    let refresh_btn: Button<'_> = header_button("🔄");
                    let refresh: Response = ui.add(refresh_btn);

                    if refresh.clicked() {
                        self.display_country = false;
                        self.display_settings = false;
                        self.internal_update();
                    }
                }

                let mut pass: bool = false;
                let srch_btn: Button<'_> = header_button("🔍");
                let search: Response = ui.add(srch_btn);

                if self.display_search {
                    self.search_topic = None;
                    let srch_box: egui::text_edit::TextEditOutput = TextEdit::singleline(&mut self.search).desired_width(150.).show(ui);
                    if search.clicked() || (srch_box.response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter))) {
                        if !self.search.is_empty() {
                            self.category = Category::Search;
                            self.internal_update();
                            self.search_topic = Some(self.search.to_string());
                        }

                        self.display_search = false;
                        pass = true;
                    }
                }

                if search.clicked() && !pass {
                    self.display_search = true;
                    self.search = "".to_string();
                }
            });
        });

        
        if self.display_country {
            let country_label: RichText = enrich (
                &self.country.stringify(),
                &TextStyle::Button,
                &TextSize::Small
            );


            // TODO: we need new solution. menu_button does what I want, but it doesn't minimize self, or better put, "unclick" itself.
            

            // How about settings button sets a state to add new frame up above or below? 
            // What if settings button is menu buttton?
            // Settings button -> night mode as text, country as sub_button/sub_menu, and text_size as sub_button/Sub_menu?
            // It could be a state: if show, pass state to update to add second menu, else if not show, display separator seen below?


            egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui,
                country_label,
                |ui: &mut Ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        
                    for i in country_vec() {
                        if ui
                            .selectable_value(
                                &mut self.country,
                                i,
                                i.stringify()
                            ).clicked() {
                                self.internal_update();
                            }
                    }
                    });
                }
            );
            });
        }

        ui.add(Separator::default());

        if self.category == Category::Search && self.search_topic.is_some() {
            let srch: RichText = RichText::new(format!("\nSearching for: {}", self.search_topic.as_ref().unwrap())).font(TextStyle::Search.set_style(&self.text_size));
            ui.add(Label::new(srch));
        } else {
            self.search_topic = None;
        }
    }

    // Footer function
    fn render_footer(&self, ui: &mut Ui) {
        ui.vertical_centered(
            |ui| {
                // Add padding
                ui.add_space(PADDING);

                // Create text
                let api_src_txt: RichText = enrich(
                    "API Source: https://newsapi.org/",
                    &TextStyle::Button,
                    &TextSize::Small
                );
                let my_txt: RichText = enrich(
                    "Check out more of my work at https://github.com/RicoNoSuave",
                    &TextStyle::Button,
                    &TextSize::Small
                );
                let egui_txt: RichText = enrich("Built with egui",
                    &TextStyle::Button,
                    &TextSize::Small
                );


                // Set URL color to regular text
                ui.style_mut().visuals.hyperlink_color = ui
                    .style_mut()
                    .visuals
                    .text_color();

                // Add text
                ui.add(Label::new(api_src_txt));
                ui.hyperlink_to(my_txt, "https://github.com/RicoNoSuave");
                ui.hyperlink_to(egui_txt, "https://github.com/emilk/egui");
            }
        );
    }
}

impl App for Interface {
    // Helper to make sure we don't paint anything behind rounded corners
    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array()
    }

    // Update function
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        // Set font and image loaders at startup
        if self.startup {
            install_image_loaders(ctx);
            self.configure_fonts(ctx);
            self.startup = false;
        }

        // Create custom frame, passing function call for rendering body
        custom_window_frame(ctx, frame, 
            |ui: &mut Ui| {
                // Render header
                self.render_header(ui);

                // Render news area
                ScrollArea::vertical()
                    .show(ui, 
                        |ui: &mut Ui| {
                            self.render_ui(ui);
                        }
                    );

                // Render footer
                TopBottomPanel::bottom("footer")
                    .show(ctx,
                        |ui: &mut Ui| {
                            self.render_footer(ui);
                    }
                );
            }
        );
    }
}

fn main() -> Result<(), eframe::Error> {
    // Load settings
    let settings: NativeOptions = settings();

    // Run app
    run_native(
        "Prometheus",
        settings,
        Box::new(
            |_cc: &CreationContext<'_>|
            Box::new(
                Interface::new()
            )
        )
    )
}

// Icon constructor
fn icon() -> Option<IconData> {
    let icon_path: &Path = Path::new("./logo/prometheus_icon.png");
    let icon_bytes: Result<Vec<u8>, Error> = read(icon_path);
    let icon: Result<IconData, ImageError> =
        IconData::try_from_png_bytes(&icon_bytes.unwrap());
    Some(icon.unwrap())
}

// Settings helper
fn settings() -> NativeOptions {
    // Construct icon for display
    let icon: Option<IconData> = icon();

    NativeOptions {
        decorated: false,
        icon_data: icon,
        initial_window_size: Some(Vec2::new(540., 960.)),
        transparent: true,
        ..Default::default()
    }
}

// Helper to generate RichText
fn enrich(string: &str, style: &TextStyle, size: &TextSize) -> RichText {
    RichText::new(string).font(style.set_style(size))
}

// Helper for header buttons
fn header_button(string: &str) -> Button {
    Button::new(
        enrich(
            string,
            &TextStyle::StaticButton,
            &TextSize::Large))
}

// Helper for generating combo box for category
fn category_vec() -> Vec<Category> {
    vec![
        Category::Business,
        Category::Entertainment,
        Category::General,
        Category::Health,
        Category::Science,
        Category::Search,
        Category::Sports,
        Category::Technology
    ]
}

// Helper for generating combo box for country
fn country_vec() -> Vec<Country> {
    vec![
        Country::Argentina,
        Country::Australia,
        Country::Austria,
        Country::Belgium,
        Country::Brazil,
        Country::Bulgaria,
        Country::Canada,
        Country::China,
        Country::Colombia,
        Country::Cuba,
        Country::Czechia,
        Country::Egypt,
        Country::France,
        Country::Germany,
        Country::Greece,
        Country::HongKong,
        Country::Hungary,
        Country::India,
        Country::Indonesia,
        Country::Ireland,
        Country::Israel,
        Country::Italy,
        Country::Japan,
        Country::Latvia,
        Country::Lithuania,
        Country::Malaysia,
        Country::Mexico,
        Country::Morocco,
        Country::Netherlands,
        Country::NewZealand,
        Country::Nigeria,
        Country::Norway,
        Country::Philippines,
        Country::Poland,
        Country::Portugal,
        Country::Romania,
        Country::Russia,
        Country::SaudiArabia,
        Country::Serbia,
        Country::Singapore,
        Country::Slovakia,
        Country::Slovenia,
        Country::SouthAfrica,
        Country::SouthKorea,
        Country::Sweden,
        Country::Switzerland,
        Country::Taiwan,
        Country::Thailand,
        Country::Turkey,
        Country::Ukraine,
        Country::UnitedArabEmirates,
        Country::UnitedKingdom,
        Country::UnitedStates,
        Country::Venezuela
    ]
}

// Helper for frame buttons
fn frame_button(string: &str) -> Button {
    Button::new(
        enrich(
            string,
            &TextStyle::Button,
            &TextSize::Large)
        )
}

// Custom Frame generator
fn custom_window_frame(ctx: &Context, frame: &mut Frame, add_body: impl FnOnce(&mut Ui)) {
    // Create frame
    let panel_frame: egui::Frame = egui::Frame {
        fill: ctx.style().visuals.window_fill(),
        rounding: 5.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(),
        ..Default::default()
    };

    // Create ;entral panel in frame
    CentralPanel::default().frame(panel_frame).show(ctx,
        |ui: &mut Ui| {
            // Get native max rectangle size
            let app_rect: Rect = ui.max_rect();

            // Create title bar rectangle that fills with height
            let title_bar_height: f32 = 28.0;
            let title_bar_rect: Rect = {
                let mut rect: Rect = app_rect;
                rect.max.y = rect.min.y + title_bar_height;
                rect
            };

            // Generate title bar into frame
            title_bar_ui(ui, frame, title_bar_rect);

            // Create content rectangle within frame (shrunk to make outline)
            let content_rect: Rect = {
                let mut rect: Rect = app_rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(4.0);

            // Create child ui to handle body
            let mut content_ui: Ui = ui
                .child_ui(content_rect, *ui.layout());

            // pass child ui back to update
            add_body(&mut content_ui);
        }
    );
}

// Create and load title bar
fn title_bar_ui( ui: &mut Ui, frame: &mut Frame, title_bar_rect: Rect) {
    // Get native ui paint color
    let painter: &Painter = ui.painter();

    // Create title bar response object
    let title_bar_response: Response = ui
        .interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // Apply paint color to title rect
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        "",
        FontId::proportional(20.),
        ui.style().visuals.text_color()
    );

    // Add line under title bar separating from body
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Track title bar responses
    if title_bar_response.double_clicked() {
        frame.set_maximized(!frame.info().window_info.maximized);
    } else if title_bar_response.is_pointer_button_down_on() {
        frame.drag_window();
    }

    // Load objects into title bar
    ui.allocate_ui_at_rect(
        title_bar_rect,
         |ui: &mut Ui| {
            // Add vertical padding
            ui.add_space(PADDING);

            // Add menu bar for same row
            egui::menu::bar(ui,
                |ui: &mut Ui| {
                    // Generate left side first
                    ui.with_layout(
                        Layout::left_to_right(Align::Center),
                        |ui: &mut Ui| {
                            // Create logo object
                            let logo: Image<'_> = Image::new(
                                include_image!(
                                    "../logo/prometheus_logo.png"
                                )
                            );

                            // Add padding to left for square
                            ui.add_space(PADDING);

                            // Add logo
                            ui.add(logo);
                        }
                    );

                    // Generate right side
                    ui.with_layout(
                        Layout::right_to_left(Align::Center),
                        |ui: &mut Ui| {
                            // Add spacing between objects
                            ui.spacing_mut().item_spacing.x = 10.0;

                            // Hide frames around objects
                            ui.visuals_mut().button_frame = false;

                            // Move in from right side for square
                            ui.add_space(PADDING);

                            // Call helper function to load
                            close_maximize_minimize(ui, frame);
                        }
                    );
                }
            );
        }
    );
}

fn close_maximize_minimize (ui: &mut Ui, frame: &mut Frame) {
    // Create objects
    let close_btn: Button<'_> = frame_button("❌");
    let max_btn = frame_button("🗗");
    let min_btn: Button<'_> = frame_button("🗕");

    // Load close button
    let close: Response = ui.add(close_btn)
        .on_hover_text("Close");

    // Handle close button response
    if close.clicked() {
        frame.close();
    }

    // Split maximize button response based on current frame size
    // If maximized, normalize, else maximize
    if frame.info().window_info.maximized {
        // Load maximize button
        let maximize: Response = ui.add(max_btn)
            .on_hover_text("Restore");

        // Handle maximize button response
        if maximize.clicked() {
            frame.set_maximized(false);
        }
    } else {
        // Load maximize button
        let maximize: Response = ui.add(max_btn)
            .on_hover_text("Maximize");

        // Handle maximize button response
        if maximize.clicked() {
            frame.set_maximized(true);
        }
    }

    // Load minimize button
    let minimize: Response = ui.add(min_btn)
        .on_hover_text("Minimize");

    // Handle minimize response
    if minimize.clicked() {
        frame.set_minimized(true);
    }
}