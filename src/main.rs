extern crate iron;
extern crate router;
#[macro_use] extern crate log;
extern crate bodyparser;
extern crate persistent;
extern crate urlencoded;
extern crate regex;

use std::process::Command;
use std::collections::HashMap;
use persistent::Read;
use iron::prelude::*;
use iron::status;
use urlencoded::{UrlEncodedQuery,UrlEncodedBody};
use router::Router;
use regex::Regex;
use iron::Headers;
use iron::mime::Mime;

#[derive(Debug)]
struct Config {
    webhook_url: String,
    slack_token: String,
    app_name: String,
    route_base: String, 
    listen_port: String,
    listen_host: String,
}

impl Config {
    fn new_from_env_or_default() -> Config {
	let app_name = match std::env::var("APP_NAME") {
	    Ok(v) => String::from(v),
	    Err(_) => String::from("el_tasko"),
	};
	let app_name_uppercase = app_name.to_uppercase();
        let webhook_url = match std::env::var(format!("{}_WEBHOOK_URL", app_name_uppercase)) {
	    Ok(v) => String::from(v),
	    Err(_) => String::from("https://hooks.slack.com/services/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"),
        }; 
        let slack_token = match std::env::var(format!("{}_TOKEN", app_name_uppercase)) {
            Ok(v) => String::from(v),
	    Err(_) => String::from("NONE")
	};
	let route_base = match std::env::var(format!("{}_ROUTE_BASE", app_name_uppercase)) {
	    Ok(v) => String::from(v),
	    Err(_) => String::from(&app_name[..])
	};
	let listen_port = match std::env::var(format!("{}_LISTEN_PORT", app_name_uppercase)) {
	    Ok(v) => String::from(v),
	    Err(_) => String::from("3000")
	};
	let listen_host = match std::env::var(format!("{}_LISTEN_HOST", app_name_uppercase)) {
	    Ok(v) => String::from(v),
	    Err(_) => String::from("0.0.0.0")
	};
	Config {
	    webhook_url: webhook_url,
	    slack_token: slack_token,
	    app_name: app_name,
	    route_base: route_base,
	    listen_port: listen_port,
	    listen_host: listen_host,
	}
    }  
}


fn help_handler(req: &mut Request) -> IronResult<Response> {
    //let content_type = iron::headers::ContentType::json();
    let content_type = "application/json".parse::<Mime>().unwrap();
    let text_help = format!("Info on how to work with bot\nIt supports following commands:\n\t/show_help - print this help\n\t/add - add task to remind. Internally it will use `at` to schedule time. Syntax: `/add \"Text you want to send (IMPORTANT!! Text must be INSIDE quotes)\" remind interval[e.g. 10min, 1hour, 1year]\"`. You can also use command like this: `/add Some text you wish to send` and in this case text will be send immediately. You will receive response on result of this operation.\n\t/list - will show output of atq. It will not be informative in fact, but it's added here to optimize this function in future.");
    Ok(Response::with((content_type, status::Ok, text_help)))
}


fn list_handler(req: &mut Request) -> IronResult<Response> {
    let result = match Command::new("sh").arg("-c").arg("atq").output() {
        Ok(v) => format!("{}", String::from_utf8(v.stdout).unwrap()),
	Err(e) => format!("Error occured: {}", e.to_string()),
    };
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, result)))
}

fn add_handler(req: &mut Request) -> IronResult<Response> {
    let CONFIG: Config = Config::new_from_env_or_default();
    println!("Config is {:?}", CONFIG);
    let headers = &req.headers;
    info!("Headers are: {:?}", headers);
    let body = req.get_ref::<UrlEncodedBody>();
    let parsed_body = body.ok().unwrap();
    info!("body is {:?}", parsed_body);
    let re = Regex::new(r#"(?P<text>["']?[\w\s]+["']?)(?P<type>\s+[a-zA-Z]+\s+)?(?P<interval>[a-z0-9]+)?"#).unwrap();

    let message: String = parsed_body["text"].iter().map(ToString::to_string).collect();
    let regex_captures = re.captures(&message).unwrap();
    let msg_to_sent = if regex_captures.len() > 2 {
        &regex_captures.name("text").unwrap().as_str()
    } else {
        &message[..]
    };
    let cmd2 = if regex_captures.name("interval").is_some() && regex_captures.name("type").is_some() {
	    // Only remind currently supported, but it could change later
	   format!("| at now + {}", regex_captures.name("interval").unwrap().as_str()) 
	} else {
	    String::from("| at now")
	};
    let cmd_curl = format!("echo 'curl -v -XPOST -H \"Content-Type: application/json\" \"{}\" -d \"{{\\\"text\\\": \\\"{}\\\"}}\" 1>>/home/sattel/logs/wtf.log 2>&1' ",
                   CONFIG.webhook_url, msg_to_sent.replace(r#"""#, r#""#));
    let cmd_to_exec = format!("{} {}", cmd_curl, cmd2);
    println!("{}", cmd_to_exec);
    let output = Command::new("sh")
                          .arg("-c")
                          .arg(cmd_to_exec)
                          .output();
    println!("output is {:?}", output);
    
    Ok(Response::with((status::Ok, match output {
                                                  Ok(v) => format!("Status OK. stdout: {:?}, stderr: {:?}", String::from_utf8(v.stdout), String::from_utf8(v.stderr)),
						  Err(e) => e.to_string(),
						})))
}

fn main() {
   let CONFIG: Config = Config::new_from_env_or_default();
   //let add_handler_with_config = add_handler(
   println!("Config is {:?}", CONFIG);
   let mut router = Router::new();
   router.post(format!("/{}/show_help", CONFIG.app_name), help_handler, "help");
   router.post(format!("/{}/list", CONFIG.app_name), list_handler, "list");
   router.post(format!("/{}/add", CONFIG.app_name), add_handler, "add");
   Iron::new(router).http(format!("{}:{}", CONFIG.listen_host, CONFIG.listen_port)).unwrap();
}
