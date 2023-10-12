//! News API

// Dependencies
use chrono::{
    Datelike,
    Local,
    Timelike
};
use serde::Deserialize;
use thiserror;
use ureq;
use url::Url;

// Async Dependency
#[cfg(feature = "async")]
use reqwest::{
    Client,
    Method
};

// Base URL
const BASE_URL: &str = "https://newsapi.org/v2";

/// Private Endpoint enum
#[derive(Clone, Copy)]
enum Endpoint {
    Everything,
    TopHeadlines
}

// to_string() implementation
impl ToString for Endpoint {
    // Cast to string for ownership upon return
    fn to_string(&self) -> String {
        match self {
            Self::Everything => "everything".to_string(),
            Self::TopHeadlines => "top-headlines".to_string()
        }
    }
}

/// Private Country enum
#[derive(Clone, Copy)]
pub enum Country {
    Argentina,
    Australia,
    Austria,
    Belgium,
    Brazil,
    Bulgaria,
    Canada,
    China,
    Colombia,
    Cuba,
    Czechia,
    Egypt,
    France,
    Germany,
    Greece,
    HongKong,
    Hungary,
    India,
    Indonesia,
    Ireland,
    Israel,
    Italy,
    Japan,
    Latvia,
    Lithuania,
    Malaysia,
    Mexico,
    Morocco,
    Netherlands,
    NewZealand,
    Nigeria,
    Norway,
    Philippines,
    Poland,
    Portugal,
    Romania,
    Russia,
    SaudiArabia,
    Serbia,
    Singapore,
    Slovakia,
    Slovenia,
    SouthAfrica,
    SouthKorea,
    Sweden,
    Switzerland,
    Taiwan,
    Thailand,
    Turkey,
    Ukraine,
    UnitedArabEmirates,
    UnitedKingdom,
    UnitedStates,
    Venezuela
}

// to_string() implementation
impl ToString for Country {
    // Cast to string for ownership upon return
    fn to_string(&self) -> String {
        match self {
            Self::Argentina => "ar".to_string(),
            Self::Australia => "au".to_string(),
            Self::Austria => "at".to_string(),
            Self::Belgium => "be".to_string(),
            Self::Brazil => "br".to_string(),
            Self::Bulgaria => "bg".to_string(),
            Self::Canada => "ca".to_string(),
            Self::China => "cn".to_string(),
            Self::Colombia => "co".to_string(),
            Self::Cuba => "cu".to_string(),
            Self::Czechia => "cz".to_string(),
            Self::Egypt => "eg".to_string(),
            Self::France => "fr".to_string(),
            Self::Germany => "de".to_string(),
            Self::Greece => "gr".to_string(),
            Self::HongKong => "hk".to_string(),
            Self::Hungary => "hu".to_string(),
            Self::India => "in".to_string(),
            Self::Indonesia => "id".to_string(),
            Self::Ireland => "ie".to_string(),
            Self::Israel => "il".to_string(),
            Self::Italy => "it".to_string(),
            Self::Japan => "jp".to_string(),
            Self::Latvia => "lv".to_string(),
            Self::Lithuania => "lt".to_string(),
            Self::Malaysia => "my".to_string(),
            Self::Mexico => "mx".to_string(),
            Self::Morocco => "ma".to_string(),
            Self::Netherlands => "nl".to_string(),
            Self::NewZealand => "nz".to_string(),
            Self::Nigeria => "ng".to_string(),
            Self::Norway => "no".to_string(),
            Self::Philippines => "ph".to_string(),
            Self::Poland => "pl".to_string(),
            Self::Portugal => "pt".to_string(),
            Self::Romania => "ro".to_string(),
            Self::Russia => "ru".to_string(),
            Self::SaudiArabia => "sa".to_string(),
            Self::Serbia => "rs".to_string(),
            Self::Singapore => "sg".to_string(),
            Self::Slovakia => "sk".to_string(),
            Self::Slovenia => "si".to_string(),
            Self::SouthAfrica => "za".to_string(),
            Self::SouthKorea => "kr".to_string(),
            Self::Sweden => "se".to_string(),
            Self::Switzerland => "ch".to_string(),
            Self::Taiwan => "tw".to_string(),
            Self::Thailand => "th".to_string(),
            Self::Turkey => "tr".to_string(),
            Self::Ukraine => "ua".to_string(),
            Self::UnitedArabEmirates => "ae".to_string(),
            Self::UnitedKingdom => "gb".to_string(),
            Self::UnitedStates => "us".to_string(),
            Self::Venezuela => "ve".to_string()
        }
    }
}

/// Private Category enum
#[derive(Clone, Copy, PartialEq)]
pub enum Category {
    Business,
    Entertainment,
    General,
    Health,
    Science,
    Search,
    Sports,
    Technology
}

// to_string() implementation
impl ToString for Category {
    // Cast to string for ownership upon return
    fn to_string(&self) -> String {
        match self {
            Self::Business => "Business".to_string(),
            Self::Entertainment => "Entertainment".to_string(),
            Self::General => "Top Headlines".to_string(),
            Self::Health => "Health".to_string(),
            Self::Science => "Science".to_string(),
            Self::Search => "Search".to_string(),
            Self::Sports => "Sports".to_string(),
            Self::Technology => "Technology".to_string()
        }
    }
}

/// Public Error Enum
// Strings explain source of error
#[derive(thiserror::Error, Debug)]
pub enum NewsAPIError {
    #[error("Unable to fetch articles at this time.\n\nCheck your internet connection.")]
    RequestFailed(#[from] ureq::Error),

    #[error("Oops! Unable to read the news! Try again soon.")]
    FailedResponseToJson(#[from] std::io::Error),

    #[error("Url parsing failed. Please contact creator.")]
    UrlParsing(#[from] url::ParseError),

    #[error("Not a Prometheus error.\nwww.newsapi.org error: {0}")]
    BadRequest(String),

    #[error("Async request failed")]
    #[cfg(feature = "async")]
    AsyncRequestFailed(#[from] reqwest::Error),
}

/// NewsAPI call Struct
// Contains API Key, Category, country, and search string
struct NewsAPI {
    api_key: String,
    category: Category,
    country: Country,
    search: String
}

impl NewsAPI {
    /// new() Constructor
    fn new(
        api_key: String,
        category: Category,
        country: Country,
        search: String
        )
        -> NewsAPI {
        NewsAPI {
            api_key: api_key,
            category: category,
            country: country,
            search: search,
        }
    }

    /// URL Constructor
    fn prepare_url(&self) -> Result<String, NewsAPIError> {
        // Base URL object
        let url: Result<Url, url::ParseError> = Url::parse(BASE_URL);

        match url {
            Ok(mut good_url) => {
                // Begin request
                let query: String;

                // If search
                if self.search.is_empty() {
                    good_url.path_segments_mut().unwrap().push(&Endpoint::TopHeadlines.to_string());
                    let country: String = format!("country={}", self.country.to_string());
                    if self.category != Category::General {
                        let category = format!("category={}", self.category.to_string());
                        query = format!("{}&{}", country, category);
                    } else {
                        query = format!("{}", country);
                    }

                } else {
                    good_url.path_segments_mut().unwrap().push(&Endpoint::Everything.to_string());
                    // Put search in query
                    query = format!("q={}&sortBy=popularity", self.search);
                }

                // Add query to url
                good_url.set_query(Some(&query));

                return Ok(good_url.to_string())
            },
            Err(e) => return Err(NewsAPIError::UrlParsing(e))
        }
    }

    /// Synchronous Caller
    fn fetch(&self) -> Result<NewsAPIResponse, NewsAPIError> {
        // Get url
        let url: Result<String, NewsAPIError> = self.prepare_url();

        // Match url validity
        match url {
            // If valid url
            Ok(good_url) => {
                // Construct request
                let req: ureq::Request = ureq::get(&good_url)
                    .set("Authorization", &self.api_key);

                // Execute request
                // Map possible errors
                let response: Result<ureq::Response, NewsAPIError> = req
                    .call()
                    .map_err(|e| NewsAPIError::RequestFailed(e));

                // Match response
                match response {
                    // If valid response
                    Ok(result) => {
                        // Unpack into json
                        let json_response: Result<NewsAPIResponse, NewsAPIError> =
                            result
                            .into_json::<NewsAPIResponse>()
                            .map_err(|e| NewsAPIError::FailedResponseToJson(e));

                        // match jsonified response
                        match json_response {
                            // if parsed into json correctly
                            Ok(news_api_response) => {

                                // Match the status field of that json
                                match news_api_response.status.as_str() {
                                    // If status is ok
                                    "ok" => return Ok(news_api_response),

                                    // Else if status is not ok, map
                                    _ => return Err(Self::map_response_err(news_api_response))
                                }
                            },
                            // Else if not parsed into json correctly
                            Err(e) => return Err(e)
                        }
                    },

                    // Else if invalid response
                    Err(e) => return Err(e)
                };
            },

            // Else if invalid url
            Err(e) => return Err(e)
        }
    }

    /// Asynchronous Caller
    #[cfg(feature = "async")]
    pub async fn fetch_async(&self) -> Result<NewsAPIResponse, NewsAPIError> {
        // get url
        let url: Result<String, NewsAPIError> = self.prepare_url();

        match url {
            // If Url is good
            Ok(_) => {
                // Use Reqwest client
                let client: Client = reqwest::Client::new();

                // Build request
                let request: Result<reqwest::Request, NewsAPIError> = client.request(Method::GET, url.unwrap())
                    .header("Authorization", &self.api_key)
                    .build()
                    .map_err(|e| NewsAPIError::AsyncRequestFailed(e));

                match request {
                    // If request is valid
                    Ok(_) => {
                        // Execute request
                        let response: Result<reqwest::Response, NewsAPIError> = client
                            .execute(request.unwrap())
                            .await
                            .map_err(|e| NewsAPIError::AsyncRequestFailed(e));

                        match response {
                            // If response was valid
                            Ok(_) => {
                                // Convert response to json
                                let news_api_response: Result<NewsAPIResponse, NewsAPIError> = response
                                    .unwrap()
                                    .json()
                                    .await
                                    .map_err(|e| NewsAPIError::AsyncRequestFailed(e));

                                // Match newsAPI.org errors
                                match news_api_response {
                                    // If response is valid json
                                    Ok(_) => {
                                        // match unwrapped response.status as string
                                        match news_api_response.as_ref().unwrap().status.as_str() {
                                            // If status is ok
                                            "ok" => return news_api_response,

                                            // Else if status is not ok, map
                                            _ => return Err(Self::map_response_err(response.unwrap()))
                                        }
                                    }
                                    Err(e) => return Err(e)
                                }
                            },
                            Err(e) => return Err(e)
                        }
                    },
                    // Else return error
                    Err(e) => return Err(e)
                }
            },
            // Else return error
            Err(e) => return Err(e)
        }
    }

    /// NewsAPI.org Error Handler
    fn map_response_err(response: NewsAPIResponse) -> NewsAPIError {
        let code: Option<String> = response.code;
        let message: Option<String> = response.message;
        if code.is_some() && message.is_some() {
            let error: String = format!("{} - {}", code.unwrap(), message.unwrap());
            NewsAPIError::BadRequest(error)
        } else {
            NewsAPIError::BadRequest("Unknown error".to_string())
        }
    }
}

/// NewsAPI.org return struct
#[derive(Deserialize, Debug)]
pub struct NewsAPIResponse {
    status: String,
    code: Option<String>,
    message: Option<String>,
    articles: Vec<NewsCard>
}

impl NewsAPIResponse {
    pub fn new(api_key: String, category: &Category, country: &Country, search: &str) -> Result<Vec<NewsCard>, NewsAPIError> {
        let newsapi: NewsAPI = NewsAPI::new(api_key, category.to_owned(), country.to_owned(), search.to_string());
        let response: Result<Self, NewsAPIError> = newsapi.fetch();

        #[cfg(feature = "async")]
        {return newsapi.fetch_async().await}

        match response {
            Ok(_) => {
                return Ok(response.unwrap().articles)
            },
            Err(e) => return Err(e)
        }
    }
}

/// Article Struct
#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct NewsCard {
    author: Option<String>,
    content: Option<String>,
    publishedAt: String,
    description: Option<String>,
    title: String,
    url: String,
}

impl NewsCard {
    // Getters
    pub fn author(&self) -> Option<&String> {
        self.author.as_ref()
    }

    pub fn content(&self) -> Option<&String> {
        self.content.as_ref()
    }

    pub fn date(&self) -> String {
        let date_time: ISODate = ISODate::new(&self.publishedAt);
        date_time.to_string()
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn url(&self) -> &str {
        &self.url
    }

}

struct ISODate {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    timezone: String,
    us: bool
}

impl ToString for ISODate {
    fn to_string(&self) -> String {
        if self.us {
            format!("{}, {} {}, {} - {} {}",
                self.get_day(),
                self.get_month(),
                self.day,
                self.year,
                self.get_time(),
                self.timezone
            )
        } else {
            format!("{}, {} {} {} - {} {}",
                self.get_day(),
                self.day,
                self.get_month(),
                self.year,
                self.get_time(),
                self.timezone
            )
        }

        
    }
}

impl ISODate {
    fn new(iso: &str) -> Self {
        // Create variables for reading
        let mut year: i32 = 0;
        let mut month: i32 = 0;
        let mut day: i32 = 0;
        let mut hour: i32 = 0;
        let mut minute: i32 = 0;

        // Pull chars into numbers
        let mut value: String = "".to_string();
        let mut int_val: i32;

        let mut index: i32 = 0;

        for c in iso.chars() {
            if c >= '0' && c <= '9' {
                value.push(c);
            } else {
                if index >= 0 && index <= 4 {
                    int_val = value.parse::<i32>().unwrap();
                    match index {
                        0 => year = int_val,
                        1 => month = int_val,
                        2 => day = int_val,
                        3 => hour = int_val,
                        4 => minute = int_val,
                        _ => {}
                    }

                    index += 1;
                    value = "".to_string();
                } else {
                    break;
                }
            }
        }

        // THIS LETS ME KNOW USERS TIMEZONE
        // Get current utc offset
        let current_utc_offset: i32 = Local::now()
            .offset()
            .local_minus_utc() / 3600;
        // Get whether it is currently dst
        let current_dst: bool = Self::curr_dst(&current_utc_offset);

        // Get user's static timezone
        let article_offset: i32;
        if current_dst && current_utc_offset >= -7 && current_utc_offset <= -4 {
            article_offset = current_utc_offset - 1;
        } else {
            article_offset = current_utc_offset;
        }

        let us: bool = article_offset >= -8 && article_offset <= -5;
        let timezone: String = Self::timezone(article_offset);

        // Create ISODate object
        let mut iso_date: ISODate = ISODate {
            year: year,
            month: month,
            day: day,
            hour: hour,
            minute: minute,
            timezone: timezone,
            us: us
        };

        // Reset the current values with the utc offset
        iso_date.utc_offset(article_offset);

        iso_date
    }

    fn curr_dst(offset: &i32) -> bool {
        let current: chrono::DateTime<Local> = Local::now();
        let year: i32 = current.year();
        let month: i32 = current.month().try_into().unwrap();
        let day: i32 = current.day().try_into().unwrap();
        let hour: i32 = current.hour().try_into().unwrap();

        let start: i32 = ((2 - 5 * ( year - 1968) / 4 ) % 7) + 8;
        let end: i32 = start - 7;

        if (month > 3 && month < 11) ||
            (month == 3 && day > start) ||
            (month == 3 && day == start && hour >= 2) ||
            (month == 11 && day < end) ||
            (month == 11 && day == end && hour < 2) {
            if offset >= &-7 && offset <= &-4 {
                return true;
            }
        }

        return false;
    }

    fn timezone(utc_offset: i32) -> String {
        match utc_offset {
            -5 => "Eastern".to_string(),
            -6 => "Central".to_string(),
            -7 => "Mountain".to_string(),
            -8 => "Pacific".to_string(),
            _ => {
                if utc_offset < 0 {
                    return format!("UTC{}", utc_offset);
                } else if utc_offset > 0 {
                    return format!("UTC+{}", utc_offset);
                } else {
                    return "UTC".to_string();
                }
            }
        }
    }

    fn utc_offset(&mut self, utc_offset: i32) {

        // Create reflective hour variable
        let mut hour: i32 = self.hour;
        hour += utc_offset;

        // If this would rewind to the previous day
        if  hour < 0 {
            // If this date would rewind to the last of the previous month
            if self.day - 1 == 0 {
                // If this month would rewind to the previous December
                if self.month - 1 == 0 {
                    self.month = 12;
                    self.year -= 1;
                } else {
                    // Else roll back month
                    self.month -= 1;
                }

                // Regardless, day rolls over so set day to max of rolled-back month
                self.day = Self::max_day(&self.month, &self.year);

            // Else if it doesn't rewind day to previous month
            } else {
                // Just roll back day
                self.day -= 1;
            }

            // Get new hour of previous day
            hour += 24;

        // else if this would advance to the next day
        } else if hour > 23 {
            // If this would advance to the next month
            if !self.day_valid(&(self.day + 1)) {
                // If the next month was January
                if self.month + 1 > 12 {
                    self.month = 1;
                    self.year += 1;

                // Else just roll forward month
                } else {
                    self.month -= 1;
                }

                self.day = 1;
            // Else just roll forward day
            } else {
                self.day += 1;
            }

            // Get new hour of next day
            hour -= 24;
        }

    self.hour = hour;
    }

    fn max_day(month: &i32, year: &i32) -> i32 {
        match month {
            1 => return 31,
            2 => {
                if year % 4 == 0 {
                    return 29;
                } else {
                    return 28;
                }
            },
            3 => return 31,
            4 => return 30,
            5 => return 31,
            6 => return 30,
            7 => return 31,
            8 => return 31,
            9 => return 30,
            10 => return 31,
            11 => return 30,
            12 => return 31,
            _ => return 31
        }
    }

    fn day_valid(&self, day: &i32) -> bool {
        match self.month {
            1 => return day <= &31,
            2 => {
                if self.year % 4 == 0 {
                    return day <= &29
                } else {
                    return day <= &28
                }
            },
            3 => return day <= &31,
            4 => return day <= &30,
            5 => return day <= &31,
            6 => return day <= &30,
            7 => return day <= &31,
            8 => return day <= &31,
            9 => return day <= &30,
            10 => return day <= &31,
            11 => return day <= &30,
            12 => return day <= &31,
            _ => return day <= &31
        }
    }

    fn get_month(&self) -> String {
        match self.month {
            1 => "January".to_string(),
            2 => "February".to_string(),
            3 => "March".to_string(),
            4 => "April".to_string(),
            5 => "May".to_string(),
            6 => "June".to_string(),
            7 => "July".to_string(),
            8 => "August".to_string(),
            9 => "September".to_string(),
            10 => "October".to_string(),
            11 => "November".to_string(),
            12 => "December".to_string(),
            _ => "Neveruary".to_string()
        }
    }

    fn get_day(&self) -> String {
        // Convert month and day to day of year
        let mut day_of_year: i32 = 0;
        let mut index: i32 = 2;
        while index <= self.month {
            match index {
                2 => day_of_year += 31,
                3 => {
                    if self.year % 4 == 0 {
                        day_of_year += 29;
                    } else {
                        day_of_year+= 28;
                    }
                },
                4 => day_of_year += 31,
                5 => day_of_year += 30,
                6 => day_of_year += 31,
                7 => day_of_year += 30,
                8 => day_of_year += 31,
                9 => day_of_year += 31,
                10 => day_of_year += 30,
                11 => day_of_year += 31,
                12 => day_of_year += 30,
                _ => {}
            }

            index += 1;
        }

        day_of_year += self.day;

        let first_of_year: i32 = ((( 3 + 5 * (self.year - 1968)) / 4) % 7) + 1;
        let day: i32 = (first_of_year + day_of_year - 2) % 7 + 1;

        match day {
            1 => return "Monday".to_string(),
            2 => return "Tuesday".to_string(),
            3 => return "Wednesday".to_string(),
            4 => return "Thursday".to_string(),
            5 => return "Friday".to_string(),
            6 => return "Saturday".to_string(),
            7 => return "Sunday".to_string(),
            _ => return "Someday".to_string()
        }
    }

    fn get_time(&self) -> String {
        let minute_str: String;
        if self.minute < 10 {
            minute_str = format!("0{}", self.minute);
        } else {
            minute_str = format!("{}", self.minute);
        }

        if self.hour > 12 {
            return format!("{}:{} p.m.", (self.hour - 12), minute_str);
        } else {
            return format!("{}:{} a.m.", self.hour, minute_str);
        }
    }
}
