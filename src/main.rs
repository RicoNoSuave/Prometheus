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
        text_edit::TextEditOutput,
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
use serde::{
    Deserialize,
    Serialize
};
use std::{
    env::var,
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

// Constant padding
const PADDING: f32 = 5.0;

// Enum for text size
#[derive(Clone, Copy, PartialEq)]
enum TextSize {
    Large,
    Medium,
    Small,
    XLarge
}

// String for menu
impl ToString for TextSize {
    fn to_string(&self) -> String {
        match self {
            TextSize::Large => "Large".to_string(),
            TextSize::Medium => "Medium".to_string(),
            TextSize::Small => "Small".to_string(),
            TextSize::XLarge => "Extra Large".to_string()
        }
    }
}

// Enum for adding TextSize to text
enum TextStyle {
    Heading,
    StaticButton,
    Search,
    Title,
    Body,
    Button
}

// Sets text_size to all text
impl TextStyle {
    fn set_style(&self, size: &TextSize) -> FontId {
        let modifier: f32;
        match size {
            TextSize::Large => modifier = 4.0,
            TextSize::Medium => modifier = 2.0,
            TextSize::Small => modifier = 0.0,
            TextSize::XLarge => modifier = 6.0
        };

		match self {
			Self::Heading => FontId::new(30.0, Proportional),
            Self::Search => FontId::new(16.0 + modifier, Proportional),
            Self::StaticButton => FontId::new(20.0, Proportional),
            Self::Title => FontId::new(12.0 + modifier, Proportional),
			Self::Body => FontId::new(10.0 + modifier, Proportional),
			Self::Button => FontId::new(10.0 + modifier, Proportional),
		}
    }
}

// State struct
// #[derive(Serialize, Deserialize)]
struct RunState {
    article: Option<NewsCard>,
    country_menu: bool,
    night_mode: bool,
    searchbar: bool,
    search_topic: Option<String>,
    startup: bool,
    text_size_menu: bool,
    text_size: TextSize
}

// State-changing functions
impl RunState {
    fn new() -> Self {
        Self {
            article: None,
            country_menu: false,
            night_mode: false,
            searchbar: false,
            search_topic: None,
            startup: true,
            text_size: TextSize::Small,
            text_size_menu: false
        }
    }

    fn hide_menus(&mut self) {
        self.country_menu = false;
        self.text_size_menu = false;
    }

    fn is_article(&self) -> bool {
        self.article.is_some()
    }

    fn get_article(&self) -> NewsCard {
        self.article.clone().unwrap()
    }

    fn set_article(&mut self, article: Option<NewsCard>) {
        self.article = article;
    }

    fn is_country_menu(&self) -> bool {
        self.country_menu
    }

    fn toggle_country_menu(&mut self) {
        self.country_menu = !self.country_menu;
    }

    fn is_night_mode(&self) -> bool {
        self.night_mode
    }

    fn toggle_night_mode(&mut self) {
        self.night_mode = !self.night_mode;
    }

    fn get_color(&self) -> Color32 {
        if self.night_mode {
            Color32::from_rgb(255,255,255)
        } else {
            Color32::from_rgb(0, 0, 0)
        }
    }

    fn is_searchbar(&self) -> bool {
        self.searchbar
    }

    fn toggle_searchbar(&mut self) {
        self.searchbar = !self.searchbar;
    }

    fn is_search_topic(&self) -> bool {
        self.search_topic.is_some()
    }

    fn get_search_topic(&self) -> String {
         self.search_topic.clone().unwrap()
    }

    fn set_search_topic(&mut self, topic: Option<String>) {
        self.search_topic = topic;
    }

    fn is_startup(&self) -> bool {
        self.startup
    }

    fn toggle_startup(&mut self) {
        self.startup = false;
    }

    fn get_text_size(&self) -> &TextSize {
        &self.text_size
    }

    fn set_text_size(&mut self, text_size: TextSize) {
        self.text_size = text_size;
    }

    fn is_text_size_menu(&self) -> bool {
        self.text_size_menu
    }

    fn toggle_text_size_menu(&mut self) {
        self.text_size_menu = !self.text_size_menu;
    }
}

// NewsAPI struct
struct NewsAPI {
    category: Category,
    country: Country,
    news: Result<Vec<NewsCard>, NewsAPIError>,
    search: String
}

// NewsAPI functions
impl NewsAPI {
    fn new() -> Self {
        // Get api key
        dotenv().unwrap();
        let api_key: String = var("API_KEY").unwrap();

        // Create new self
        Self {
            category: Category::General,
            country: Country::UnitedStates,
            news: NewsAPIResponse::new(
                api_key,
                &Category::General,
                &Country::UnitedStates,
                ""
            ),
            search: "".to_string()
        }
    }

    fn update(&mut self) {
        // Get api key
        dotenv().unwrap();
        let api_key: String = var("API_KEY").unwrap();

        self.news = NewsAPIResponse::new(
            api_key,
            &self.category,
            &self.country,
            &self.search
        )
    }

    fn get_category(&self) -> Category {
        self.category
    }

    fn set_category(&mut self, category: Category) {
        self.category = category;
        if category != Category::Search {
            self.update();
        }
    }

    fn get_country(&self) -> Country {
        self.country
    }

    fn set_country(&mut self, country: Country) {
        self.country = country;
        self.update();
    }

    fn get_api_call(&self) -> &Result<Vec<NewsCard>, NewsAPIError> {
        &self.news
    }

    fn get_news(&self) -> Vec<NewsCard> {
        self.news.as_ref().unwrap().to_owned()
    }

    fn get_search(&self) -> &str {
        &self.search
    }

    fn set_search(&mut self, search: &str) {
        self.search = search.to_string();
        self.update();
        self.category = Category::Search;
    }
}

// Save state values
#[derive(Serialize, Deserialize)]
struct SaveState {
    night_mode: bool,
    text_small: bool,
    text_medium: bool,
    text_large: bool
}

// Prometheus structure
struct Prometheus {
    api_response: NewsAPI,
    search_string: String,
    state: RunState
}

impl Prometheus {
    fn new() -> Self {
        // Generate self
        Self {
            api_response: NewsAPI::new(),
            search_string: "".to_string(),
            state: RunState::new()
        }
    }

    // helper to set fonts
    fn configure_fonts(&mut self, ctx: &Context) {
        // Get default font definitions
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
            .get_mut(
                &FontFamily::Proportional
            )
            .unwrap()
            .insert(
                0,
                "MesloLGS".to_owned()
            );

        // Set MesloLGS as fallback
        font_def
            .families
            .get_mut(
                &FontFamily::Monospace
            )
            .unwrap()
            .push(
                "MesloLGS".to_owned()
            );

        // Load new font definitions to context
        ctx.set_fonts(font_def);
    }

    // UI controller
    fn render_ui(&mut self, ui: &mut Ui) {
        // If there is an article
        if self.state.is_article() {
            // Render the article
            self.render_article(ui);
        }

        else {
            // Parse news
            match self.api_response.get_api_call() {
                Ok(_e) => self.render_news(ui),
                Err(e) =>{
                    ui.add(Label::new(format!("{}", e.to_string())));
                }
            }
        }
    }

    fn render_article(&mut self, ui: &mut Ui) {
        // Get article
        let news: NewsCard = self.state.get_article();

        // Render title
        ui.add_space(PADDING);
        let title: RichText = enrich(
            news.trim_title(),
            &TextStyle::Title,
            self.state.get_text_size()
        );
        ui.add(Label::new(title));

        // Render author
        if news.is_author() {
            ui.add_space(PADDING);
            let author: RichText = enrich(
                &format!("By: {}", news.author()),
                &TextStyle::Body,
                self.state.get_text_size()
            );
            ui.add(Label::new(author));
        }

        // Render date
        ui.add_space(PADDING);
        let date: RichText = enrich(
            &news.date(),
            &TextStyle::Body,
            self.state.get_text_size()
        );
        ui.add(Label::new(date));

        // Render content
        ui.add_space(PADDING);
        let content: RichText = enrich(
            &format!("{}\n", news.content()),
            &TextStyle::Body,
            self.state.get_text_size()
        );
        ui.add(Label::new(content));

        // Add links and buttons below
        menu::bar(
            ui,
            |ui: &mut Ui| {
                // Start from left
                ui.with_layout(
                    Layout::left_to_right(
                        Align::Center),
                    |ui: &mut Ui| {
                        // Create return button
                        ui.add_space(PADDING);
                        let ret_btn_txt: RichText = enrich(
                            "Return to News",
                            &TextStyle::Button,
                            &self.state.get_text_size()
                        );
                        let ret_btn: Response = ui
                        .add(
                            Button::new(ret_btn_txt)
                        );

                        // Handle return buttton
                        if ret_btn.clicked() {
                            self.state.set_article(None);
                        }
                    }
                );

                // Now the right
                ui.with_layout(
                    Layout::right_to_left(
                        Align::Center),
                    |ui: &mut Ui| {
                        // Create hyperlink
                        ui.add_space(PADDING);
                        let url_txt = enrich(
                            "Read More Online ⤴",
                            &TextStyle::Button,
                            &self.state.get_text_size()
                        );
                        let link_btn: Response = ui.add(
                            Hyperlink::from_label_and_url(
                                url_txt, news.url()
                            )
                        );
                        if link_btn.clicked() {
                            self.state.set_article(None);
                        }
                    }
                );
            }
        );
    }

    // Render News
    fn render_news(&mut self, ui: &mut Ui) {
        // For all news
        for newscard in self.api_response.get_news() {
            // If the article exits
            if newscard.title() != "[Removed]" {
                // Render title
                ui.add_space(PADDING);
                let title: RichText = enrich(
                    &format!("▶ {}", newscard.title()),
                    &TextStyle::Title,
                    &self.state.get_text_size()
                );
                ui.colored_label(
                    self.state.get_color(),
                    title
                );

                // Render description
                if newscard.is_description() {
                    ui.add_space(PADDING);
                    let description: RichText = enrich(
                        newscard.description(),
                        &TextStyle::Body,
                        &self.state.get_text_size()
                    );
                    ui.add(Label::new(description));
                }

                // Add spacing for vertical
                ui.add_space(PADDING);

                // Render bottom menu
                menu::bar(
                    ui,
                    |ui: &mut Ui| {
                        // Start on left if there is content
                        if newscard.is_content() {
                            ui.with_layout(
                                Layout::left_to_right(Align::Center),
                                |ui: &mut Ui| {
                                    // Add read button
                                    let read_btn_txt: RichText = enrich(
                                        "Read Article",
                                        &TextStyle::Button,
                                        &self.state.text_size
                                    );
                                    let read: Response = ui.add(
                                        Button::new(
                                            read_btn_txt
                                        )
                                    );
                                    if read.clicked() {
                                        self.state.set_article(
                                            Some(newscard.to_owned())
                                        );
                                    }
                                }
                            );
                        }

                        // Now do right
                        ui.with_layout(
                            Layout::right_to_left(Align::Center),
                            |ui: &mut Ui| {
                                // Add hyperlink
                                ui.add_space(PADDING);
                                let url: RichText = enrich(
                                    "Read Online ⤴",
                                    &TextStyle::Button,
                                    &self.state.get_text_size()
                                );
                                ui.add(Hyperlink::from_label_and_url(
                                    url,
                                    newscard.url()
                                    )
                                );
                            }
                        );

                    }
                );

                // Add separator
                ui.add(Separator::default());
            }
        }
    }

    // Header function
    fn render_header(&mut self, ui: &mut Ui) {
        // Set header to single row
        menu::bar(
            ui,
            |ui| {
                // Start from left
                ui.with_layout(Layout::left_to_right(Align::Center),
                    |ui| {
                        // Side buffer
                        ui.add_space(PADDING);

                        // Get current category
                        let category: RichText = enrich(
                            &self.api_response.get_category().to_string(),
                            &TextStyle::Heading,
                            &self.state.get_text_size()
                        );

                        // Create combo box of category
                        let mut new_cat: Category = self
                            .api_response
                            .get_category(); 
                        let cat_box: Response =
                            ComboBox::from_label("")
                                .selected_text(category)
                                .width(180.)
                                .show_ui(
                                    ui,
                                    |ui: &mut Ui| {
                                        // Iterate
                                        for cat in category_vec() {
                                            // Get response
                                            if ui
                                                .selectable_value(
                                                    &mut new_cat,
                                                    cat,
                                                    cat.to_string()
                                                )
                                                .clicked() {
                                                    // Clear search for update
                                                    self.api_response.set_search("");

                                                    // Remove article if any
                                                    self.state.set_article(None);

                                                    // Hide search bar
                                                    if self.state.is_searchbar() {
                                                        self.state.toggle_searchbar();
                                                    }

                                                    // Submit and update
                                                    self
                                                        .api_response
                                                        .set_category(new_cat);
                                                }
                                        }
                                    }
                                ).response;
                        
                        if cat_box.clicked() {
                            self.state.hide_menus();
                        }
                    }
                );

                // Now do right side, shifted down and right to bottom
                ui.with_layout(
                    Layout::right_to_left(
                        Align::Max),
                        |ui: &mut Ui| {
                            // Side padding
                            ui.add_space(PADDING);

                            // Create settings button as menu button
                            let settings_txt: RichText = enrich(
                                "🔧",
                                &TextStyle::StaticButton,
                                &TextSize::Large);
                            let settings_response: Response = menu::menu_button(
                                    ui, 
                                    settings_txt, 
                                    |ui: &mut Ui| {
                                        //Create country button to set display state
                                        let country_menu: Response = ui.button("Country");
                                        if country_menu.clicked() && !self.state.is_country_menu() {
                                            self.state.toggle_country_menu();

                                            // If opened, close text menu
                                            if self.state.is_text_size_menu() {
                                                self.state.toggle_text_size_menu();
                                            }
                                        } else if country_menu.clicked() && self.state.is_country_menu() {
                                            self.state.toggle_country_menu();
                                        }

                                        // If display state
                                        if self.state.is_country_menu() {
                                            ScrollArea::vertical()
                                                .max_height(500.0)
                                                .show(
                                                    ui,
                                                    |ui: &mut Ui| {
                                                        let mut new_country: Country = self
                                                            .api_response
                                                            .get_country();
                                                        for country in country_vec() {
                                                            if ui
                                                                .selectable_value(
                                                                    &mut new_country,
                                                                    country,
                                                                    country.stringify())
                                                                .clicked() {
                                                                    self.api_response.set_category(Category::General);
                                                                    self.state.set_article(None);
                                                                    self.state.toggle_country_menu();
                                                                    self.api_response.set_country(country);
                                                                }
                                                        }
                                                    }
                                                );
                                        }

                                        // Create text size button to set display state
                                        let text_menu: Response = ui.button("Text Size");
                                        if text_menu.clicked() && !self.state.is_text_size_menu() {
                                            self.state.toggle_text_size_menu();

                                            // If country menu is open, close it
                                            if self.state.is_country_menu() {
                                                self.state.toggle_country_menu();
                                            }
                                        } else if text_menu.clicked() && self.state.is_text_size_menu() {
                                            self.state.toggle_text_size_menu();
                                        }

                                        if self.state.is_text_size_menu() {
                                            let mut new_size: TextSize = self.state.get_text_size().to_owned();
                                            for i in text_size_vec() {
                                                if ui.selectable_value(
                                                    &mut new_size,
                                                    i,
                                                    i.to_string()
                                                    )
                                                    .clicked() {
                                                        self.state.toggle_text_size_menu();
                                                        self.state.set_text_size(new_size);
                                                    }
                                            }
                                        }
                                    }
                                ).response;

                            if settings_response.clicked() && self.state.is_searchbar() {
                                self.state.toggle_searchbar();
                            }

                            // Create night mode button
                            let night_mode: RichText;
                            if self.state.is_night_mode() {
                                night_mode = enrich(
                                    "🌙",
                                    &TextStyle::StaticButton,
                                    &TextSize::Large
                                );
                            } else {
                                night_mode = enrich(
                                    "🌞",
                                    &TextStyle::StaticButton,
                                    &TextSize::Large
                                );
                            }

                            let nmode_btn: Response = ui.add(
                                Button::new(night_mode));

                            if nmode_btn.clicked() {
                                self.state.toggle_night_mode();

                                // Close other menus
                                self.state.hide_menus();
                            }

                            // Create refresh button
                            let refresh_btn: Button<'_> = header_button("🔄");
                            let refresh: Response = ui.add(refresh_btn);

                            // Handle refresh call
                            if refresh.clicked() {
                                self.api_response.update();

                                self.state.hide_menus();
                                self.state.set_article(None);
                                if self.state.is_searchbar() {
                                    self.state.toggle_searchbar();
                                }
                            }

                            // Create search button
                            let srch_btn: Button<'_> = header_button("🔍");
                            let search: Response = ui.add(srch_btn);

                            // Handle search call
                            // If clicked while off, turn on, else if clicked while on, turn off.
                            if search.clicked() && !self.state.is_searchbar() {
                                self.state.toggle_searchbar();
                                self.state.hide_menus();
                            } else if search.clicked() && self.state.is_searchbar() {
                                self.state.toggle_searchbar();
                                self.state.hide_menus();
                            }

                            // Handle search bar display
                            if self.state.is_searchbar() {
                                // Create search box
                                let srch_box: TextEditOutput = 
                                    TextEdit::singleline(
                                        &mut self.search_string)
                                        .desired_width(150.).show(ui);

                                // If search is clicked or enter pressed
                                if (
                                    search.clicked()
                                    || (srch_box.response.lost_focus()
                                        && ui.input(
                                            |i: &egui::InputState|
                                            i.key_pressed(Key::Enter))
                                        )
                                    ) && !self.search_string.is_empty() {
                                    // Remove article if any
                                    self.state.set_article(None);

                                    // Set category to search
                                    self.api_response.set_category(Category::Search);

                                    // Execute search
                                    self.api_response.set_search(&self.search_string);

                                    // Set new search as topic
                                    self.state.set_search_topic(Some(self.search_string.to_owned()));

                                    // Clear search string
                                    self.search_string = "".to_string();

                                    // toggle search bar
                                    self.state.toggle_searchbar();
                                }

                                // If empty string submitted through box
                                if srch_box.response.lost_focus()
                                    && ui.input(
                                        |i: &egui::InputState|
                                        i.key_pressed(Key::Enter))
                                    && self.search_string.is_empty()
                                    && self.api_response.get_search().is_empty() {
                                    self.state.toggle_searchbar();
                                }
                        }
                    }
                );
            }
        );

        // Add separator
        ui.add(Separator::default());

        // If the category is search and there is a search topic
        if self.api_response.get_category() == Category::Search && self.state.is_search_topic() {
            // Create Search notification
            let srch: RichText = enrich(
            &format!("\nSearching for: {}", self.state.get_search_topic()),
            &TextStyle::Search,
            &self.state.get_text_size());
            ui.add(Label::new(srch));
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

impl App for Prometheus {
    // Helper to make sure we don't paint anything behind rounded corners
    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array()
    }

    // Update function
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        // Set font and image loaders at startup
        if self.state.is_startup() {
            install_image_loaders(ctx);
            self.configure_fonts(ctx);
            self.state.toggle_startup();
        }

        // Enable night mode
        if self.state.is_night_mode() {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
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
                Prometheus::new()
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
        Category::Sports,
        Category::Technology
    ]
}

// Helper for generating vec for country
fn country_vec() -> Vec<Country> {
    vec![
        Country::Argentina,
        Country::Australia,
        Country::Austria,
        Country::Belgium,
        Country::Brazil,
        Country::Bulgaria,
        Country::Canada,
        Country::Colombia,
        Country::Cuba,
        Country::Czechia,
        Country::France,
        Country::Germany,
        Country::Greece,
        Country::Hungary,
        Country::India,
        Country::Indonesia,
        Country::Ireland,
        Country::Italy,
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
        Country::Sweden,
        Country::Switzerland,
        Country::Turkey,
        Country::Ukraine,
        Country::UnitedKingdom,
        Country::UnitedStates,
        Country::Venezuela
    ]
}

// Helper for generating vector for text size
fn text_size_vec() -> Vec<TextSize> {
    vec![
        TextSize::Small,
        TextSize::Medium,
        TextSize::Large,
        TextSize::XLarge
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