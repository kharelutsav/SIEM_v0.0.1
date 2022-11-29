use std::{time, collections::HashMap, fs};
use regex::Regex;
#[path = "cn/log_parser.rs"] mod log_parser;
#[path = "cn/log_normalizer.rs"] mod log_normalizer;

#[derive(Debug)]
struct Helper {
    _norm_id: String,
    _device_category: String,
    _internal_regex_objects: Vec<Regex>
}


fn csv_to_hashmap (path_of_file: &str) -> HashMap<String, String> { // Csv to hashmap <String, String>
    let mut new_hashmap: HashMap<String, String> = HashMap::new();
    let read_file = fs::read_to_string(path_of_file).unwrap();
    for lines in read_file.clone().split_terminator("\n") {
        let pair = lines.split(",").collect:: <Vec<&str>> ();
        new_hashmap.insert(pair[0].to_string(), pair[1].to_string());
    };
    new_hashmap
}

fn main() {
    let active_regex = json::parse(&fs::read_to_string("src/data/json/active_regex.json").unwrap()).unwrap().members().map(|x| x.to_string()).collect:: <Vec<String>>();
    let all_normalizers = json::parse(&fs::read_to_string("src/data/json/regex_details.json").unwrap()).unwrap();
    let mut regex_objects: Vec<Regex> = vec![];
    let mut _taxonomy_map: HashMap<&String, HashMap<String, String>> = HashMap::new();
    let mut _type_map: HashMap<&String, HashMap<String, String>> = HashMap::new();
    let mut normalizers: HashMap<&String, Helper> = HashMap::new();

    for regex in &active_regex {

        regex_objects.push(Regex::new(&regex).unwrap());
        _taxonomy_map.insert(regex, csv_to_hashmap(&all_normalizers[regex]["config_taxonomy"].to_string()));
        _type_map.insert(regex, csv_to_hashmap(&all_normalizers[regex]["config_type"].to_string()));

        normalizers.insert(regex, Helper{
            _norm_id: all_normalizers["norm_id"].to_string(),
            _device_category: all_normalizers["device_category"].to_string(),
            _internal_regex_objects: all_normalizers[regex]["possible_regex"].members().map(|x|Regex::new(&x.to_string()).unwrap()).collect:: <Vec<Regex>>(),
        });

        if regex == &active_regex[active_regex.len()-1] {

            let log = r#"Aug 16 06:44:32 10.255.231.54 3c76668474611520f20a249c797ed76d: [Tue Aug 16 06:44:31.312303 2022] [workflow:alert] [140755:139645092910848] ICX Engine raised event: {"eventUid":"77fb17103ded43b8a5a4c0f26352b9cd","tokens":{"date":1617111871312001,"eventType":"security","engineUid":"icxEngine","engineName":"ICX Engine","attackFamily":"File Inclusion","riskLevel":70,"riskLevelOWASP":7.0,"cwe":"CWE-98","severity":5,"resolveType":"Default Resolve","part":"Multiple","icxPolicyName":"ICX - Configuration CD33 (Prod) v3.35.0","icxPolicyUid":"74c8811217755edf658f89d968ce5593","icxRuleName":"Remote file include by Get Vars","icxRuleUid":"hhhha8b382ef37a66f0b620c39adbbba","matchingParts":[{"part":"Path","partValue":"/index.php","partValueOperator":"pattern","partValuePatternUid":"PhpUriPattern_00445","partValuePatternName":"Php URI","partValuePatternVersion":"00445","partValueMatch":".php"},{"part":"Var_GET","partKey":"view","partKeyOperator":"regexp","partKeyPattern":".*","partKeyMatch":"view","partValue":"http://www.google.com","partValueOperator":"pattern","partValuePatternUid":"RFIProprietaryPattern_00421","partValuePatternName":"Remote file include","partValuePatternVersion":"00421","partValueMatch":"http://www.google.com","attackFamily":"File Inclusion","riskLevel":70,"riskLevelOWASP":7.0,"cwe":"CWE-98"}],"reason":"ICX Engine: File Inclusion in Var_GET 'view'","securityExceptionConfigurationUids":["cc0dc59fd2d276531ec0613213e236d7"]}}"#;
            // let log = r#"regex is present in the {"logAlertUid":"74b0069f374147c9b16ec40b933e6cf7","@timestamp":"1592391200524","timestamp":"1592391200524","request":{"body":"","cookies":[],"headers":[{"key":"Host","value":"contribution.gironde.fr"},{"key":"Accept","value":"*/*"},{"key":"User-agent","value":"Mozilla/5.0 (compatible; DotBot/1.1; http://www.opensiteexplorer.org/dotbot, help@moz.com)"},{"key":"Accept-Charset","value":"utf-8;q=0.7,iso-8859-1;q=0.2,*;q=0.1"}],"hostname":"contribution.gironde.fr","ipDst":"10.255.231.55","ipSrc":"216.244.66.246","method":"GET","path":"/jcms/cgw_78390/jean-guy-perriere","portDst":443,"protocol":"HTTP/1.1","query":"","requestUid":"Xun2IINvcPHPO3AWuL5FOgAAAOI"},"context":{"tags":"","applianceName":"SRV-DENY-PROD2","applianceUid":"11919eb977f2bc2e9acebd23efac2eb7","backendHost":"10.255.234.60","backendPort":443,"reverseProxyName":"RP-PROD-2","reverseProxyUid":"0fd6cf80f8f3905cd890eb37aa432aa8","tunnelName":"","tunnelUid":"NoUID","workflowName":"Block and log unknown hostname","workflowUid":"BlockUnknownHostnameLog"},"events":[{"eventUid":"57bb18ff06104157945e47d5394b0acc","tokens":{"date":1592391200524552,"eventType":"security","engineUid":"custom","engineName":"Custom","attackFamily":"No Attack Family","riskLevel":50,"riskLevelOWASP":0.0,"cwe":"-","severity":5,"resolveType":"No Resolve","part":"No Part","customMessage":"Block unknown hostname contribution.gironde.fr","reason":"Custom: Block unknown hostname contribution.gironde.fr"}}]}"#;

            let start_time = time::Instant::now();
            
            'second: for regex in &regex_objects {
                if regex.is_match(log) {
                    let normalizer = normalizers.get(&regex.to_string()).unwrap();
                    for internal_regex in &normalizer._internal_regex_objects{
                        if internal_regex.is_match(log) {  
                            let one = internal_regex.find(log).unwrap();
                            log_normalizer::Normalizer {
                                parsed_log: log_parser::Parser{
                                    raw_log: json::parse(&log[one.start()..one.end()]).unwrap(),
                                }.build_stru_log(),
                                taxonomy_mapping: _taxonomy_map.get(&internal_regex.to_string()).unwrap().clone(),
                                type_mapping: _type_map.get(&internal_regex.to_string()).unwrap().clone()
                            }.normalize();
                            break 'second;
                        }
                    }
                }
            }

            println!("{:?}", start_time.elapsed());
        }
    }
}