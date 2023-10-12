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
        Context,
        FontData,
        FontDefinitions,
        FontFamily::{
            self,
            Proportional
        },
        FontId,
        Hyperlink,
        Image,
        ImageSource,
        Key,
        Label,
        Layout,
        RichText,
        ScrollArea,
        Separator,
        Style,
        TextEdit,
        TextStyle::*,
        TopBottomPanel,
        Ui,
        Vec2, ComboBox
    },
    Frame,
    NativeOptions,
    run_native
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
            Self::White => Color32::from_rgb(255, 255, 255)
        }
    }
}

enum Font {
    MesloLGS,
    Mono
}

impl Font {
    fn define (&self) -> FontDefinitions {
        let mut font_def: FontDefinitions = FontDefinitions::default();
        match self {
            Self::MesloLGS => {
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

                font_def
            },
            _ => FontDefinitions::default()
        }
    }
}

enum TextSize {
    Large,
    Medium,
    Small
}

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

struct Interface {
    category: Category,
    country: Country,
    display_search: bool,
    font: Font,
    news: Result<Vec<NewsCard>, NewsAPIError>,
    search: String,
    search_state: Option<String>,
    text_size: TextSize,
    close: bool
}

impl Interface {
    fn new() -> Self {
        dotenv().unwrap();

        let api_key: String = std::env::var("API_KEY").unwrap();
        Self {
            category: Category::General,
            country: Country::UnitedStates,
            display_search: false,
            font: Font::MesloLGS,
            news: NewsAPIResponse::new(
                    api_key,
                    &Category::General,
                    &Country::UnitedStates,
                    ""
                ),
            search: "".to_string(),
            search_state: None,
            text_size: TextSize::Small,
            close: false
        }
    }

    fn configure_fonts(&self, ctx: &Context) {
        ctx.set_fonts(self.font.define());
    }

    fn set_text_style(&self, ctx: &Context) {
        let mut style: Style = (*ctx.style()).clone();
    
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Body, FontId::new(14.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();
        ctx.set_style(style);
    }

    fn internal_update(&mut self) {
        dotenv().unwrap();
        let api_key: String = std::env::var("API_KEY").unwrap();

        self.news = NewsAPIResponse::new(
            api_key,
            &self.category,
            &self.country,
            &self.search
        );
    }

    fn render_ui(&self, ui: &mut Ui) {
        match &self.news {
            Ok(_e) => self.render_news(ui),
            Err(e) =>{
                ui.add(Label::new(format!("{}", e.to_string())));
            }
        }
    }

    fn render_news(&self, ui: &mut Ui) {
        let mut iter: std::slice::Iter<'_, NewsCard> = 
            self
            .news
            .as_ref()
            .unwrap()
            .iter();

        while let Some(newscard) = iter.next() {
            if newscard.title() != "[Removed]" {
                // Pad
                ui.add_space(PADDING);

                // Create title
                let title: RichText =
                    RichText::new(
                        format!("▶ {}", newscard.title())
                    )
                    .font(
                        TextStyle::Title
                        .set_style(&self.text_size)
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
                    let description: RichText =
                        RichText::new(
                            newscard
                            .description()
                            .unwrap()
                        )
                        .font(
                            TextStyle::Body.set_style(&self.text_size)
                        );

                    // Render
                    ui.add(Label::new(description));
                }

                // Set URL color
                let url_color: Color = Color::Cyan;
                ui.style_mut()
                    .visuals.hyperlink_color = url_color.color();

                // Pad
                ui.add_space(PADDING);

                // Set style
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    // Create Link
                    let link: RichText = RichText::new("read more ⤴")
                    .font(
                        TextStyle::Button.set_style(&self.text_size)
                    );

                    ui.add(Hyperlink::from_label_and_url(link, newscard.url()));
                });

                // Add separator
                ui.add_space(PADDING);
                ui.add(Separator::default());
            }
        }
    }

    fn render_header(&mut self, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                let heading: RichText = RichText::new(self.category.to_string()).font(TextStyle::Heading.set_style(&TextSize::Large));
 
                let category: Category = self.category;

                ComboBox::from_label("")
                    .selected_text(heading)
                    .width(180.)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.category, Category::Business, Category::Business.to_string());
                        ui.selectable_value(&mut self.category, Category::Entertainment, Category::Entertainment.to_string());
                        ui.selectable_value(&mut self.category, Category::General, Category::General.to_string());
                        ui.selectable_value(&mut self.category, Category::Health, Category::Health.to_string());
                        ui.selectable_value(&mut self.category, Category::Science, Category::Science.to_string());
                        ui.selectable_value(&mut self.category, Category::Sports, Category::Sports.to_string());
                        ui.selectable_value(&mut self.category, Category::Technology, Category::Technology.to_string());
                    }
                    );

                if category != self.category {
                    self.display_search = false;
                    self.internal_update();
                }
            });

            ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                ui.add_space(PADDING);

                let mut pass: bool = false;
                let srch_btn_txt: RichText = RichText::new("🔍").font(TextStyle::StaticButton.set_style(&TextSize::Large));
                let srch_btn: egui::Response = ui.add(Button::new(srch_btn_txt));

                if self.display_search {
                    self.search_state = None;
                    let srch_box: egui::text_edit::TextEditOutput = TextEdit::singleline(&mut self.search).desired_width(150.).show(ui);
                    if srch_btn.clicked() || (srch_box.response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter))) {
                        if !self.search.is_empty() {
                            self.category = Category::Search;
                            self.internal_update();
                            self.search_state = Some(self.search.to_string());
                        }

                        self.display_search = false;
                        pass = true;
                    }
                }

                if srch_btn.clicked() && !pass {
                    self.display_search = true;
                    self.search = "".to_string();
                }
            });
        });

        ui.add(Separator::default());

        if self.category == Category::Search && self.search_state.is_some() {
            let srch: RichText = RichText::new(format!("\nSearching for: {}", self.search_state.as_ref().unwrap())).font(TextStyle::Search.set_style(&self.text_size));
            ui.add(Label::new(srch));
        }
    }

    fn render_footer(&self, ctx:&Context ) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(PADDING);
                let api_src_txt: RichText = RichText::new("API Source: https://newsapi.org/").font(TextStyle::Button.set_style(&TextSize::Small));
                let my_txt: RichText = RichText::new("Check out more of my work at https://github.com/RicoNoSuave").font(TextStyle::Button.set_style(&TextSize::Small));
                let egui_txt: RichText = RichText::new("Built with egui").font(TextStyle::Button.set_style(&TextSize::Small));


                // Set URL color
                ui.style_mut()
                    .visuals.hyperlink_color = ui.style_mut().visuals.text_color();

                ui.add(Label::new(api_src_txt));
                ui.hyperlink_to(my_txt, "https://github.com/RicoNoSuave");
                ui.hyperlink_to(egui_txt, "https://github.com/emilk/egui");
            });
        });
    }

    fn render_top_panel(&mut self, ctx: &Context) {
        // Create a TopBottomPanel widget
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(PADDING);
            ui.add_space(PADDING);
            egui::menu::bar(ui, |ui| {
                ui.add_space(PADDING);

                // Logo
                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    let image_source: ImageSource = egui::include_image!("../logo/prometheus_logo.png");
                    ui.add(Image::new(image_source));
                });

                // controls
                ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                    let close_btn_txt: RichText = RichText::new("❌").font(TextStyle::StaticButton.set_style(&TextSize::Large));
                    let settings_btn_txt: RichText = RichText::new("🔧").font(TextStyle::StaticButton.set_style(&TextSize::Large));
                    let refresh_btn_txt: RichText = RichText::new("🔄").font(TextStyle::StaticButton.set_style(&TextSize::Large));
                    let theme_btn_txt: RichText = RichText::new("🌙").font(TextStyle::StaticButton.set_style(&TextSize::Large));
                    let close_btn = ui.add(Button::new(close_btn_txt));
                    let settings_btn = ui.add(Button::new(settings_btn_txt));
                    let refresh_btn = ui.add(Button::new(refresh_btn_txt));
                    let theme_btn = ui.add(Button::new(theme_btn_txt));

                    if refresh_btn.clicked() {
                        self.internal_update();
                    }

                    if close_btn.clicked() {
                        self.close = true;
                    }

                    // TODO: Learn about states for Close, settings, and theme
                    ui.add_space(PADDING);
                });
            });
        });
    }
}

impl App for Interface {
    fn update(
        &mut self,
        ctx: &Context,
        frame: &mut Frame
    )
    {
        self.set_text_style(ctx);
        self.configure_fonts(ctx);

        egui_extras::install_image_loaders(ctx);

        // Top Panel
        self.render_top_panel(ctx);

        // Central Panel
        CentralPanel::default()
            .show(ctx, |ui: &mut Ui| {
            // Vertical Scroll
            self.render_header(ui);
            ScrollArea::vertical()
                .show(ui, |ui: &mut Ui| {
                    self.render_ui(ui);
                });
            self.render_footer(ctx);
        });

        if self.close {
            frame.close();
        }
    }
}

fn main() {
    let settings: NativeOptions = NativeOptions {
        initial_window_size: Some(Vec2::new(540., 960.)),
        // decorated: false,
        drag_and_drop_support: true,
        ..Default::default()
    };

    let _app: Result<(), eframe::Error> =
        run_native(
            "Prometheus",
            settings,
            Box::new(
                |_cc: &CreationContext<'_>|
                Box::new(
                    Interface::new()
                )
            )
        );

    println!("We made it here, just to prove that the segfault is with the WSL drivers.");
}