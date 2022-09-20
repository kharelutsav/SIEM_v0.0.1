mod cn;
mod repo;
mod models;

use cn::_instance;
use std::time::Instant;
use repo::mongo_repo::Mongo;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = Mongo::init().await;
    let (_processor, _parser, _normalizer) = _instance::init(db).await;
    let raw_logs = r#"regex is present in the {"logAlertUid":"74b0069f374147c9b16ec40b933e6cf7","@timestamp":"1592391200524","timestamp":"1592391200524","request":{"body":"","cookies":[],"headers":[{"key":"Host","value":"contribution.gironde.fr"},{"key":"Accept","value":"*/*"},{"key":"User-agent","value":"Mozilla/5.0 (compatible; DotBot/1.1; http://www.opensiteexplorer.org/dotbot, help@moz.com)"},{"key":"Accept-Charset","value":"utf-8;q=0.7,iso-8859-1;q=0.2,*;q=0.1"}],"hostname":"contribution.gironde.fr","ipDst":"10.255.231.55","ipSrc":"216.244.66.246","method":"GET","path":"/jcms/cgw_78390/jean-guy-perriere","portDst":443,"protocol":"HTTP/1.1","query":"","requestUid":"Xun2IINvcPHPO3AWuL5FOgAAAOI"},"context":{"tags":"","applianceName":"SRV-DENY-PROD2","applianceUid":"11919eb977f2bc2e9acebd23efac2eb7","backendHost":"10.255.234.60","backendPort":443,"reverseProxyName":"RP-PROD-2","reverseProxyUid":"0fd6cf80f8f3905cd890eb37aa432aa8","tunnelName":"","tunnelUid":"NoUID","workflowName":"Block and log unknown hostname","workflowUid":"BlockUnknownHostnameLog"},"events":[{"eventUid":"57bb18ff06104157945e47d5394b0acc","tokens":{"date":1592391200524552,"eventType":"security","engineUid":"custom","engineName":"Custom","attackFamily":"No Attack Family","riskLevel":50,"riskLevelOWASP":0.0,"cwe":"-","severity":5,"resolveType":"No Resolve","part":"No Part","customMessage":"Block unknown hostname contribution.gironde.fr","reason":"Custom: Block unknown hostname contribution.gironde.fr"}}]}
    <134>1 2022-09-06T11:20:13.335561+02:00 V000SRVBK EventsFeederImporter.Host.exe 0 42 - LEEF:1.0|Panda Security|paps|02.55.00.0000|registrym|sev=1	devTime=2022-09-06 09:20:13.833380	devTimeFormat=yyyy-MM-dd HH:mm:ss.SSS	usrName=Clinicà	domain=RECEP-VCALLOSA	src=172.16.58.33	identSrc=172.16.58.33	identHostName=RECEP-VCALLOSA	HostName=RECEP-VCALLOSA	MUID=D5CBB14638B046ACB68989780EF7C437	LocalDateTime=2022-09-02T06:09:13.833+02:00	PandaTimeStatus=2	Op=ModifyExeKey	Hash=2EAC4BE665BA052852753899A939B870	DriveType=Fixed	Path=LOCAL_APPDATA|\Microsoft\OneDrive\OneDrive.exe	ValidSig=true	Company=Microsoft Corporation	Broken=true	ImageType=EXE 64	ExeType=Unknown	Prevalence=High	PrevLastDay=Low	Cat=Goodware	MWName=	TargetPath=Personal	RegKey=\REGISTRY\USER\S-1-5-21-3275747432-1976284724-4110513935-1004_Classes\CLSID\{2e7c0a19-0438-41e9-81e3-3ad3d64f55ba}\LocalServer32?(default)\=1|"C:\Users\Clinicà\AppData\Local\Microsoft\OneDrive\OneDrive.exe" /cci /client
    Aug 16 06:44:32 10.255.231.54 3c76668474611520f20a249c797ed76d: [Tue Aug 16 06:44:31.312303 2022] [workflow:alert] [140755:139645092910848] ICX Engine raised event: {"eventUid":"77fb17103ded43b8a5a4c0f26352b9cd","tokens":{"date":1617111871312001,"eventType":"security","engineUid":"icxEngine","engineName":"ICX Engine","attackFamily":"File Inclusion","riskLevel":70,"riskLevelOWASP":7.0,"cwe":"CWE-98","severity":5,"resolveType":"Default Resolve","part":"Multiple","icxPolicyName":"ICX - Configuration CD33 (Prod) v3.35.0","icxPolicyUid":"74c8811217755edf658f89d968ce5593","icxRuleName":"Remote file include by Get Vars","icxRuleUid":"hhhha8b382ef37a66f0b620c39adbbba","matchingParts":[{"part":"Path","partValue":"/index.php","partValueOperator":"pattern","partValuePatternUid":"PhpUriPattern_00445","partValuePatternName":"Php URI","partValuePatternVersion":"00445","partValueMatch":".php"},{"part":"Var_GET","partKey":"view","partKeyOperator":"regexp","partKeyPattern":".*","partKeyMatch":"view","partValue":"http://www.google.com","partValueOperator":"pattern","partValuePatternUid":"RFIProprietaryPattern_00421","partValuePatternName":"Remote file include","partValuePatternVersion":"00421","partValueMatch":"http://www.google.com","attackFamily":"File Inclusion","riskLevel":70,"riskLevelOWASP":7.0,"cwe":"CWE-98"}],"reason":"ICX Engine: File Inclusion in Var_GET 'view'","securityExceptionConfigurationUids":["cc0dc59fd2d276531ec0613213e236d7"]}}
    <134>1 2022-09-06T11:20:13.335561+02:00 V000SRVBK EventsFeederImporter.Host.exe 0 42 - LEEF:2.0|Panda Security|paps|02.55.00.0000|registrym|^|devTimeFormat=yyyy-MMdd'T'HH:mm:ss.SSSZ^devTime=2017-10-18T11:26:03.060+0200^usrName=flastname^name=Utsav Kharel^authType=interactivePassword^src=192.168.0.1"#;
    let _raw_logs = r#"Aug 16 06:44:32 10.255.231.54 3c76668474611520f20a249c797ed76d: [Tue Aug 16 06:44:31.312303 2022] [workflow:alert] [140755:139645092910848] ICX Engine raised event: {"eventUid":"77fb17103ded43b8a5a4c0f26352b9cd","tokens":{"date":1617111871312001,"eventType":"security","engineUid":"icxEngine","engineName":"ICX Engine","attackFamily":"File Inclusion","riskLevel":70,"riskLevelOWASP":7.0,"cwe":"CWE-98","severity":5,"resolveType":"Default Resolve","part":"Multiple","icxPolicyName":"ICX - Configuration CD33 (Prod) v3.35.0","icxPolicyUid":"74c8811217755edf658f89d968ce5593","icxRuleName":"Remote file include by Get Vars","icxRuleUid":"hhhha8b382ef37a66f0b620c39adbbba","matchingParts":[{"part":"Path","partValue":"/index.php","partValueOperator":"pattern","partValuePatternUid":"PhpUriPattern_00445","partValuePatternName":"Php URI","partValuePatternVersion":"00445","partValueMatch":".php"},{"part":"Var_GET","partKey":"view","partKeyOperator":"regexp","partKeyPattern":".*","partKeyMatch":"view","partValue":"http://www.google.com","partValueOperator":"pattern","partValuePatternUid":"RFIProprietaryPattern_00421","partValuePatternName":"Remote file include","partValuePatternVersion":"00421","partValueMatch":"http://www.google.com","attackFamily":"File Inclusion","riskLevel":70,"riskLevelOWASP":7.0,"cwe":"CWE-98"}],"reason":"ICX Engine: File Inclusion in Var_GET 'view'","securityExceptionConfigurationUids":["cc0dc59fd2d276531ec0613213e236d7"]}}"#;
    // let mut stdin = io::stdin().lines();
    let start_time = Instant::now();
    for _ in 0..10000 {
        let mut raw_logs = raw_logs.lines();
        while let Some(raw_log) = raw_logs
        .next()
        {
            if let Some(id) = _processor.pre_process(&raw_log) {
                let mut str_log = _parser.parse(&id, &raw_log);
                _processor.post_process(&id, &raw_log, &mut str_log);
                let _normalized_log = _normalizer.normalize(str_log, &id);
            }
        }
    }
    println!("{:#?}", start_time.elapsed());
    Ok(())
}
