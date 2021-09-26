use std::fmt::format;

use nipper::Document;

#[tokio::main]
async fn main() {
  println!("开始去抓奇亚官方浏览器");
  let url = "https://www.chiaexplorer.com/blockchain/search"; //奇亚官方浏览器
  let response = get_url_data(url).await;
  let html = response.text().await.expect("抓网页出错");
  let chunk_js = parsing_chia_main_chunk_js(html);
  let url = format!("https://www.chiaexplorer.com{}", chunk_js);
  println!("main_chunk_js:{}", url);
  let response = get_url_data(url.as_str()).await;
  let js_content = response.text().await.unwrap();
  let api_url = parsing_js(js_content);
  println!("取到了接口域名啦:{:#?}", api_url);
}

/* 通过分析js中的apiPath2=url就是接口URL */
pub fn parsing_js(js_content: String) -> Option<String> {
  let mut api_url: Option<String> = None;
  // println!("分析JS，通过分析js中的apiPath2");
  //先用;来截断
  let mut split = js_content.split(";");
  for row_code_js in split {
    /* 这一行 ：var n={apiPath:"https://api.chiaexplorer.com/chia",apiPath2:"https://will-break-dont-ever-use-this.chiaexplorer.com"}},202:function(e,a,t){e.exports=t.p+"static/media/avatar3.d5f40349.jpg"},207:function(e,a){e.exports="data:image/jpeg */
    let api_path = "n={apiPath"; //这一行里分析
    if row_code_js.find(api_path) != None {
      // println!("就是这一行：{}", row_code_js);
      let row_split = row_code_js.split(",");
      for row in row_split {
        if row.find("apiPath2:") != None {
          // println!("处理这个:{}", row);
          let temp = row.to_string();
          let len = temp.len() - 3;
          let temp = &temp[10..len];
          // println!("这谅是找到的接口域名：{}", temp);
          api_url = Some(temp.to_string());
        }
      }
      break;
    }
  }
  api_url
}

/* 分析取得奇亚处理块的js */
pub fn parsing_chia_main_chunk_js(html: String) -> String {
  use nipper::Document;

  /* 开始分析html */
  let document = Document::from(&html);
  let mut temp = String::new();
  document.select("script").iter().for_each(|athing| {
    let href = athing.attr("src");
    match href {
      Some(urljs) => {
        // println!("找到：{}", urljs);
        temp = urljs.to_owned().to_string();
      }
      None => println!(""),
    }
    // println!("查看一下");
  });

  // println!("完成,最后一个:{}", temp);
  temp
}

pub async fn get_url_data(url: &str) -> reqwest::Response {
  use reqwest::header::HeaderMap;

  let client = reqwest::Client::new();
  let mut headers = HeaderMap::new();
  // headers.insert("ontent-Type", "application/json".parse().unwrap());
  headers.insert("user-agent", "luck-kd".parse().unwrap());
  let response = client.get(url).headers(headers).send().await.unwrap();

  if !response.status().as_str().eq("200") {
    println!("抓取网页({})出错!", url);
  }

  response
}

fn main_yl() {
  // let html = get_html();
  let html = get_html_two();
  let document = Document::from(&html);

  document.select("tr.athing").iter().for_each(|athing| {
    let title = athing.select(".title a");
    let href = athing.select(".storylink");
    println!("{}", title.text());
    let temp = href.attr("name").unwrap();
    println!("{}", href.attr("href").unwrap());
    println!("=============================");
  });

  let mut temp = String::new();
  document.select("script").iter().for_each(|athing| {
    let href = athing.attr("src");
    match href {
      Some(urljs) => {
        println!("找到：{}", urljs);
        temp = urljs.to_owned().to_string();
      }
      None => println!("这里是JS，无外部引用"),
    }
    // println!("查看一下");
  });

  println!("完成,最后一个:{}", temp);
}

fn get_html_two() -> String {
  let html = r#"
    <!doctype html><html lang="en"><head><meta charset="utf-8"/><base href="/"><link rel="shortcut icon" href="/favicon.ico"/><meta name="viewport" content="width=device-width,initial-scale=1"/><meta name="theme-color" content=000000"/><meta name="description" content="The worlds best Chia cryptocurrency blockchain explorer."/><link rel="apple-touch-icon" href="logo192.png"/><link rel="manifest" href="/manifest.json"/><title>Chia cryptocurrency blockchain explorer</title><script async src="https://www.googletagmanager.com/gtag/js?id=UA-166890627-1"></script><script>function gtag(){dataLayer.push(arguments)}window.dataLayer=window.dataLayer||[],gtag("js",new Date),gtag("config","UA-166890627-1")</script><script async src="https://coinzillatag.com/lib/display.js"></script><link href="/static/css/4.993c56e7.chunk.css" rel="stylesheet"><link href="/static/css/main.f8d540bd.chunk.css" rel="stylesheet"></head><body><noscript>You need to enable JavaScript to run this app.</noscript><div id="root"/><script>!function(e){function r(r){for(var n,a,u=r[0],f=r[1],i=r[2],s=0,p=[];s<u.length;s++)a=u[s],Object.prototype.hasOwnProperty.call(o,a)&&o[a]&&p.push(o[a][0]),o[a]=0;for(n in f)Object.prototype.hasOwnProperty.call(f,n)&&(e[n]=f[n]);for(l&&l(r);p.length;)p.shift()();return c.push.apply(c,i||[]),t()}function t(){for(var e,r=0;r<c.length;r++){for(var t=c[r],n=!0,u=1;u<t.length;u++){var f=t[u];0!==o[f]&&(n=!1)}n&&(c.splice(r--,1),e=a(a.s=t[0]))}return e}var n={},o={3:0},c=[];function a(r){if(n[r])return n[r].exports;var t=n[r]={i:r,l:!1,exports:{}};return e[r].call(t.exports,t,t.exports,a),t.l=!0,t.exports}a.e=function(e){var r=[],t=o[e];if(0!==t)if(t)r.push(t[2]);else{var n=new Promise((function(r,n){t=o[e]=[r,n]}));r.push(t[2]=n);var c,u=document.createElement("script");u.charset="utf-8",u.timeout=120,a.nc&&u.setAttribute("nonce",a.nc),u.src=function(e){return a.p+"static/js/"+({2:"polyfills-css-shim"}[e]||e)+"."+{0:"c166ad84",2:"93602ef9",5:"7c0fb866",6:"1e42370a",7:"6eedc04e",8:"02d74f95",9:"f11263b8",10:"4c845fce",11:"40e0f124",12:"bcbf3f51",13:"8a1fce5a",14:"1f082372",15:"8c192822",16:"16cd3028",17:"9a67db2e",18:"2ba7663f",19:"77055756",20:"44b72ab7",21:"9ce41df9",22:"ccd749d5",23:"bb7d00fc",24:"cc8d5a3a",25:"c11efc30",26:"ab4fdee6",27:"82021437",28:"7c998b14",29:"ae46cbec",30:"0c08f1c4",31:"190a8293",32:"2cb97416",33:"1ef17287",34:"379a12d8",35:"3e716349"}[e]+".chunk.js"}(e);var f=new Error;c=function(r){u.onerror=u.onload=null,clearTimeout(i);var t=o[e];if(0!==t){if(t){var n=r&&("load"===r.type?"missing":r.type),c=r&&r.target&&r.target.src;f.message="Loading chunk "+e+" failed.\n("+n+": "+c+")",f.name="ChunkLoadError",f.type=n,f.request=c,t[1](f)}o[e]=void 0}};var i=setTimeout((function(){c({type:"timeout",target:u})}),12e4);u.onerror=u.onload=c,document.head.appendChild(u)}return Promise.all(r)},a.m=e,a.c=n,a.d=function(e,r,t){a.o(e,r)||Object.defineProperty(e,r,{enumerable:!0,get:t})},a.r=function(e){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},a.t=function(e,r){if(1&r&&(e=a(e)),8&r)return e;if(4&r&&"object"==typeof e&&e&&e.__esModule)return e;var t=Object.create(null);if(a.r(t),Object.defineProperty(t,"default",{enumerable:!0,value:e}),2&r&&"string"!=typeof e)for(var n in e)a.d(t,n,function(r){return e[r]}.bind(null,n));return t},a.n=function(e){var r=e&&e.__esModule?function(){return e.default}:function(){return e};return a.d(r,"a",r),r},a.o=function(e,r){return Object.prototype.hasOwnProperty.call(e,r)},a.p="/",a.oe=function(e){throw console.error(e),e};var u=this.webpackJsonpchiaexplorer=this.webpackJsonpchiaexplorer||[],f=u.push.bind(u);u.push=r,u=u.slice();for(var i=0;i<u.length;i++)r(u[i]);var l=f;t()}([])</script><script src="/static/js/4.909cc370.chunk.js"></script><script src="/static/js/main.8059f6a7.chunk.js"></script></body></html>
    "#;
  html.to_string()
}

fn get_html() -> String {
  let html = r#"
    <html op="news">

    <head>
      <meta name="referrer" content="origin">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <link rel="stylesheet" type="text/css" href="news.css?EdFA4gW5lET4IyyrLeIo">
      <link rel="shortcut icon" href="favicon.ico">
      <link rel="alternate" type="application/rss+xml" title="RSS" href="rss">
      <title>Hacker News</title>
    </head>
    
    <body>
      <center>
        <table id="hnmain" border="0" cellpadding="0" cellspacing="0" width="85%" bgcolor="f6f6ef">
          <tr>
            <td bgcolor="ff6600">
              <table border="0" cellpadding="0" cellspacing="0" width="100%" style="padding:2px">
                <tr>
                  <td style="width:18px;padding-right:4px"><a href="https://news.ycombinator.com"><img src="y18.gif"
                        width="18" height="18" style="border:1px white solid;"></a></td>
                  <td style="line-height:12pt; height:10px;"><span class="pagetop"><b class="hnname"><a href="news">Hacker
                          News</a></b>
                      <a href="newest">new</a> | <a href="front">past</a> | <a href="newcomments">comments</a> | <a
                        href="ask">ask</a> | <a href="show">show</a> | <a href="jobs">jobs</a> | <a href="submit">submit</a>
                    </span></td>
                  <td style="text-align:right;padding-right:4px;"><span class="pagetop">
                      <a href="login?goto=news">login</a>
                    </span></td>
                </tr>
              </table>
            </td>
          </tr>
          <tr id="pagespace" title="" style="height:10px"></tr>
          <tr>
            <td>
              <table border="0" cellpadding="0" cellspacing="0" class="itemlist">
                <tr class='athing' id='22800516'>
                  <td align="right" valign="top" class="title"><span class="rank">1.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22800516' href='vote?id=22800516&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://www.engadget.com/2020/03/03/undead-gadgets/" class="storylink">Gadgets
                      That Refuse to Die</a><span class="sitebit comhead"> (<a href="from?site=engadget.com"><span
                          class="sitestr">engadget.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22800516">40 points</span> by <a href="user?id=davewongillies"
                      class="hnuser">davewongillies</a> <span class="age"><a href="item?id=22800516">1 hour ago</a></span>
                    <span id="unv_22800516"></span> | <a href="hide?id=22800516&amp;goto=news">hide</a> | <a
                      href="item?id=22800516">35&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22798118'>
                  <td align="right" valign="top" class="title"><span class="rank">2.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22798118' href='vote?id=22798118&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://www.nytimes.com/2020/04/06/business/arbitration-overload.html"
                      class="storylink">A new weapon in arbitration: sheer volume</a><span class="sitebit comhead"> (<a
                        href="from?site=nytimes.com"><span class="sitestr">nytimes.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22798118">412 points</span> by <a href="user?id=jseliger"
                      class="hnuser">jseliger</a> <span class="age"><a href="item?id=22798118">9 hours ago</a></span> <span
                      id="unv_22798118"></span> | <a href="hide?id=22798118&amp;goto=news">hide</a> | <a
                      href="item?id=22798118">171&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22800433'>
                  <td align="right" valign="top" class="title"><span class="rank">3.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22800433' href='vote?id=22800433&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://en.wikipedia.org/wiki/OK_Soda" class="storylink">OK Soda</a><span
                      class="sitebit comhead"> (<a href="from?site=wikipedia.org"><span
                          class="sitestr">wikipedia.org</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22800433">32 points</span> by <a href="user?id=velmu"
                      class="hnuser">velmu</a> <span class="age"><a href="item?id=22800433">2 hours ago</a></span> <span
                      id="unv_22800433"></span> | <a href="hide?id=22800433&amp;goto=news">hide</a> | <a
                      href="item?id=22800433">10&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22799407'>
                  <td align="right" valign="top" class="title"><span class="rank">4.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22799407' href='vote?id=22799407&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a
                      href="https://www.npr.org/sections/coronavirus-live-updates/2020/04/06/828187071/some-auto-insurers-are-sending-refunds-to-customers-as-crash-rate-falls"
                      class="storylink">Some auto insurers are sending refunds to customers as crash rate falls</a><span
                      class="sitebit comhead"> (<a href="from?site=npr.org"><span class="sitestr">npr.org</span></a>)</span>
                  </td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22799407">111 points</span> by <a href="user?id=prostoalex"
                      class="hnuser">prostoalex</a> <span class="age"><a href="item?id=22799407">6 hours ago</a></span>
                    <span id="unv_22799407"></span> | <a href="hide?id=22799407&amp;goto=news">hide</a> | <a
                      href="item?id=22799407">87&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22797060'>
                  <td align="right" valign="top" class="title"><span class="rank">5.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22797060' href='vote?id=22797060&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="http://www.quakejs.com/" class="storylink">QuakeJS – A Quake Port to JavaScript
                      with Emscripten</a><span class="sitebit comhead"> (<a href="from?site=quakejs.com"><span
                          class="sitestr">quakejs.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22797060">269 points</span> by <a href="user?id=zaiste"
                      class="hnuser">zaiste</a> <span class="age"><a href="item?id=22797060">11 hours ago</a></span> <span
                      id="unv_22797060"></span> | <a href="hide?id=22797060&amp;goto=news">hide</a> | <a
                      href="item?id=22797060">124&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22797688'>
                  <td align="right" valign="top" class="title"><span class="rank">6.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22797688' href='vote?id=22797688&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://blog.jessfraz.com/post/containers-zones-jails-vms/"
                      class="storylink">Containers vs. Zones vs. Jails vs. VMs (2017)</a><span class="sitebit comhead"> (<a
                        href="from?site=jessfraz.com"><span class="sitestr">jessfraz.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22797688">235 points</span> by <a href="user?id=gullyfur"
                      class="hnuser">gullyfur</a> <span class="age"><a href="item?id=22797688">10 hours ago</a></span> <span
                      id="unv_22797688"></span> | <a href="hide?id=22797688&amp;goto=news">hide</a> | <a
                      href="item?id=22797688">76&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22799817'>
                  <td align="right" valign="top" class="title"><span class="rank">7.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22799817' href='vote?id=22799817&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="http://megalomaniacbore.blogspot.com/2014/04/virtual-cpu-in-c-4001-cpu.html"
                      class="storylink">Write your own Virtual CPU in C++ (2014)</a><span class="sitebit comhead"> (<a
                        href="from?site=megalomaniacbore.blogspot.com"><span
                          class="sitestr">megalomaniacbore.blogspot.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22799817">47 points</span> by <a href="user?id=andreygrehov"
                      class="hnuser">andreygrehov</a> <span class="age"><a href="item?id=22799817">4 hours ago</a></span>
                    <span id="unv_22799817"></span> | <a href="hide?id=22799817&amp;goto=news">hide</a> | <a
                      href="item?id=22799817">1&nbsp;comment</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22796845'>
                  <td align="right" valign="top" class="title"><span class="rank">8.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22796845' href='vote?id=22796845&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://www.edge.org/response-detail/11783" class="storylink">What concept
                      would improve everybody's cognitive toolkit? Kayfabe (2011)</a><span class="sitebit comhead"> (<a
                        href="from?site=edge.org"><span class="sitestr">edge.org</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22796845">268 points</span> by <a href="user?id=aleyan"
                      class="hnuser">aleyan</a> <span class="age"><a href="item?id=22796845">11 hours ago</a></span> <span
                      id="unv_22796845"></span> | <a href="hide?id=22796845&amp;goto=news">hide</a> | <a
                      href="item?id=22796845">119&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22795189'>
                  <td align="right" valign="top" class="title"><span class="rank">9.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22795189' href='vote?id=22795189&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a
                      href="https://www.wsj.com/articles/shipping-delays-out-of-stock-items-amazon-isnt-the-only-shop-online-11586165400"
                      class="storylink">Amazon Isn’t the Only Shop Online</a><span class="sitebit comhead"> (<a
                        href="from?site=wsj.com"><span class="sitestr">wsj.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22795189">363 points</span> by <a href="user?id=prostoalex"
                      class="hnuser">prostoalex</a> <span class="age"><a href="item?id=22795189">14 hours ago</a></span>
                    <span id="unv_22795189"></span> | <a href="hide?id=22795189&amp;goto=news">hide</a> | <a
                      href="item?id=22795189">432&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22799866'>
                  <td align="right" valign="top" class="title"><span class="rank">10.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22799866' href='vote?id=22799866&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a
                      href="https://www.economist.com/briefing/2020/04/04/technology-startups-are-headed-for-a-fall"
                      class="storylink">Exit unicorns, pursued by bears</a><span class="sitebit comhead"> (<a
                        href="from?site=economist.com"><span class="sitestr">economist.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22799866">32 points</span> by <a href="user?id=Jerry2"
                      class="hnuser">Jerry2</a> <span class="age"><a href="item?id=22799866">4 hours ago</a></span> <span
                      id="unv_22799866"></span> | <a href="hide?id=22799866&amp;goto=news">hide</a> | <a
                      href="item?id=22799866">14&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22798616'>
                  <td align="right" valign="top" class="title"><span class="rank">11.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22798616' href='vote?id=22798616&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="http://cs241.cs.illinois.edu/coursebook/index.html" class="storylink">Free
                      System Programming Textbook (Illinois CS241)</a><span class="sitebit comhead"> (<a
                        href="from?site=illinois.edu"><span class="sitestr">illinois.edu</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22798616">123 points</span> by <a href="user?id=garren"
                      class="hnuser">garren</a> <span class="age"><a href="item?id=22798616">8 hours ago</a></span> <span
                      id="unv_22798616"></span> | <a href="hide?id=22798616&amp;goto=news">hide</a> | <a
                      href="item?id=22798616">4&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22798433'>
                  <td align="right" valign="top" class="title"><span class="rank">12.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22798433' href='vote?id=22798433&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a
                      href="http://philstrazzulla.com/2020/04/06/tips-for-founders-sales-lessons-from-starting-two-b2b-startups/"
                      class="storylink">Tips for Founders Sales: Lessons from Starting Two B2B Startups</a><span
                      class="sitebit comhead"> (<a href="from?site=philstrazzulla.com"><span
                          class="sitestr">philstrazzulla.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22798433">98 points</span> by <a href="user?id=pstrazzulla"
                      class="hnuser">pstrazzulla</a> <span class="age"><a href="item?id=22798433">8 hours ago</a></span>
                    <span id="unv_22798433"></span> | <a href="hide?id=22798433&amp;goto=news">hide</a> | <a
                      href="item?id=22798433">18&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22799499'>
                  <td align="right" valign="top" class="title"><span class="rank">13.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22799499' href='vote?id=22799499&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a
                      href="https://www.bloomberg.com/news/articles/2020-04-06/universities-forced-to-face-addiction-to-foreign-students-money"
                      class="storylink">Universities forced to face addiction to foreign students’ money</a><span
                      class="sitebit comhead"> (<a href="from?site=bloomberg.com"><span
                          class="sitestr">bloomberg.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22799499">48 points</span> by <a href="user?id=hhs" class="hnuser">hhs</a>
                    <span class="age"><a href="item?id=22799499">5 hours ago</a></span> <span id="unv_22799499"></span> | <a
                      href="hide?id=22799499&amp;goto=news">hide</a> | <a href="item?id=22799499">17&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22800136'>
                  <td align="right" valign="top" class="title"><span class="rank">14.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22800136' href='vote?id=22800136&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="item?id=22800136" class="storylink">Ask HN: What is your blog and why should I
                      read it?</a></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22800136">43 points</span> by <a href="user?id=jppope"
                      class="hnuser">jppope</a> <span class="age"><a href="item?id=22800136">3 hours ago</a></span> <span
                      id="unv_22800136"></span> | <a href="hide?id=22800136&amp;goto=news">hide</a> | <a
                      href="item?id=22800136">53&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22800671'>
                  <td align="right" valign="top" class="title"><span class="rank">15.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22800671' href='vote?id=22800671&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://heaphero.io/" class="storylink" rel="nofollow">Java Memory
                      Analyzer</a><span class="sitebit comhead"> (<a href="from?site=heaphero.io"><span
                          class="sitestr">heaphero.io</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22800671">6 points</span> by <a href="user?id=Ram_Lakshmanan"
                      class="hnuser">Ram_Lakshmanan</a> <span class="age"><a href="item?id=22800671">1 hour ago</a></span>
                    <span id="unv_22800671"></span> | <a href="hide?id=22800671&amp;goto=news">hide</a> | <a
                      href="item?id=22800671">3&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22799430'>
                  <td align="right" valign="top" class="title"><span class="rank">16.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22799430' href='vote?id=22799430&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://victorqribeiro.itch.io/qubes" class="storylink">Show HN: I'm releasing
                      my game for free</a><span class="sitebit comhead"> (<a href="from?site=itch.io"><span
                          class="sitestr">itch.io</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22799430">100 points</span> by <a href="user?id=atum47"
                      class="hnuser">atum47</a> <span class="age"><a href="item?id=22799430">6 hours ago</a></span> <span
                      id="unv_22799430"></span> | <a href="hide?id=22799430&amp;goto=news">hide</a> | <a
                      href="item?id=22799430">73&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22798245'>
                  <td align="right" valign="top" class="title"><span class="rank">17.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22798245' href='vote?id=22798245&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://www.despeak.com/" class="storylink">DeSpeak: Practice at public
                      speaking</a><span class="sitebit comhead"> (<a href="from?site=despeak.com"><span
                          class="sitestr">despeak.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22798245">13 points</span> by <a href="user?id=wrazo"
                      class="hnuser">wrazo</a> <span class="age"><a href="item?id=22798245">3 hours ago</a></span> <span
                      id="unv_22798245"></span> | <a href="hide?id=22798245&amp;goto=news">hide</a> | <a
                      href="item?id=22798245">discuss</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22796409'>
                  <td align="right" valign="top" class="title"><span class="rank">18.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22796409' href='vote?id=22796409&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="http://iolanguage.org/" class="storylink">The Io Language</a><span
                      class="sitebit comhead"> (<a href="from?site=iolanguage.org"><span
                          class="sitestr">iolanguage.org</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22796409">71 points</span> by <a href="user?id=pcr910303"
                      class="hnuser">pcr910303</a> <span class="age"><a href="item?id=22796409">7 hours ago</a></span> <span
                      id="unv_22796409"></span> | <a href="hide?id=22796409&amp;goto=news">hide</a> | <a
                      href="item?id=22796409">7&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22795671'>
                  <td align="right" valign="top" class="title"><span class="rank">19.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22795671' href='vote?id=22795671&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a
                      href="https://www.wsj.com/articles/foursquare-merges-with-factual-another-location-data-provider-11586193000"
                      class="storylink">Foursquare Merges with Factual</a><span class="sitebit comhead"> (<a
                        href="from?site=wsj.com"><span class="sitestr">wsj.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22795671">196 points</span> by <a href="user?id=leothekim"
                      class="hnuser">leothekim</a> <span class="age"><a href="item?id=22795671">13 hours ago</a></span>
                    <span id="unv_22795671"></span> | <a href="hide?id=22795671&amp;goto=news">hide</a> | <a
                      href="item?id=22795671">111&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22796646'>
                  <td align="right" valign="top" class="title"><span class="rank">20.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22796646' href='vote?id=22796646&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://www.spomenikdatabase.org/what-are-spomeniks" class="storylink">What Are
                      Spomeniks?</a><span class="sitebit comhead"> (<a href="from?site=spomenikdatabase.org"><span
                          class="sitestr">spomenikdatabase.org</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22796646">37 points</span> by <a href="user?id=dr_dshiv"
                      class="hnuser">dr_dshiv</a> <span class="age"><a href="item?id=22796646">5 hours ago</a></span> <span
                      id="unv_22796646"></span> | <a href="hide?id=22796646&amp;goto=news">hide</a> | <a
                      href="item?id=22796646">25&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22797089'>
                  <td align="right" valign="top" class="title"><span class="rank">21.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22797089' href='vote?id=22797089&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://github.com/ghdl/ghdl-yosys-plugin" class="storylink">VHDL support for
                      open-source FPGA toolchain YoSys</a><span class="sitebit comhead"> (<a
                        href="from?site=github.com"><span class="sitestr">github.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22797089">87 points</span> by <a href="user?id=ur-whale"
                      class="hnuser">ur-whale</a> <span class="age"><a href="item?id=22797089">11 hours ago</a></span> <span
                      id="unv_22797089"></span> | <a href="hide?id=22797089&amp;goto=news">hide</a> | <a
                      href="item?id=22797089">10&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22782259'>
                  <td align="right" valign="top" class="title"><span class="rank">22.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22782259' href='vote?id=22782259&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://basesandframes.wordpress.com/2020/04/04/even-faster-math-functions/"
                      class="storylink">Even Faster Math Functions</a><span class="sitebit comhead"> (<a
                        href="from?site=basesandframes.wordpress.com"><span
                          class="sitestr">basesandframes.wordpress.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22782259">53 points</span> by <a href="user?id=matt_d"
                      class="hnuser">matt_d</a> <span class="age"><a href="item?id=22782259">6 hours ago</a></span> <span
                      id="unv_22782259"></span> | <a href="hide?id=22782259&amp;goto=news">hide</a> | <a
                      href="item?id=22782259">1&nbsp;comment</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22800418'>
                  <td align="right" valign="top" class="title"><span class="rank">23.</span></td>
                  <td></td>
                  <td class="title"><a href="item?id=22800418" class="storylink">Corvus Robotics (YC S18) is hiring a
                      robotics QA/DevOps engineer</a></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="age"><a href="item?id=22800418">2 hours ago</a></span> | <a
                      href="hide?id=22800418&amp;goto=news">hide</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22797286'>
                  <td align="right" valign="top" class="title"><span class="rank">24.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22797286' href='vote?id=22797286&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://github.com/shiyanhui/libcsp" class="storylink">Libcsp: a fast C
                      concurrency library influenced by the CSP model</a><span class="sitebit comhead"> (<a
                        href="from?site=github.com"><span class="sitestr">github.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22797286">99 points</span> by <a href="user?id=lime66"
                      class="hnuser">lime66</a> <span class="age"><a href="item?id=22797286">10 hours ago</a></span> <span
                      id="unv_22797286"></span> | <a href="hide?id=22797286&amp;goto=news">hide</a> | <a
                      href="item?id=22797286">18&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22785100'>
                  <td align="right" valign="top" class="title"><span class="rank">25.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22785100' href='vote?id=22785100&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://avrillion.com/stf/363/How-to-Build-1-Bit-of-RAM-Using-Transistors"
                      class="storylink">How to Build 1 Bit of RAM Using Transistors</a><span class="sitebit comhead"> (<a
                        href="from?site=avrillion.com"><span class="sitestr">avrillion.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22785100">72 points</span> by <a href="user?id=ImGameDeving"
                      class="hnuser">ImGameDeving</a> <span class="age"><a href="item?id=22785100">9 hours ago</a></span>
                    <span id="unv_22785100"></span> | <a href="hide?id=22785100&amp;goto=news">hide</a> | <a
                      href="item?id=22785100">31&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22795930'>
                  <td align="right" valign="top" class="title"><span class="rank">26.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22795930' href='vote?id=22795930&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://codeberg.org/" class="storylink">Codeberg: a free, non-commercial
                      GitHub alternative</a><span class="sitebit comhead"> (<a href="from?site=codeberg.org"><span
                          class="sitestr">codeberg.org</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22795930">115 points</span> by <a href="user?id=passthejoe"
                      class="hnuser">passthejoe</a> <span class="age"><a href="item?id=22795930">13 hours ago</a></span>
                    <span id="unv_22795930"></span> | <a href="hide?id=22795930&amp;goto=news">hide</a> | <a
                      href="item?id=22795930">52&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22799263'>
                  <td align="right" valign="top" class="title"><span class="rank">27.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22799263' href='vote?id=22799263&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a
                      href="https://techcrunch.com/2020/04/06/angellist-lays-off-a-number-of-staff-and-cuts-executive-salaries/"
                      class="storylink">AngelList lays off a number of staff and cuts executive salaries</a><span
                      class="sitebit comhead"> (<a href="from?site=techcrunch.com"><span
                          class="sitestr">techcrunch.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22799263">128 points</span> by <a href="user?id=pseudolus"
                      class="hnuser">pseudolus</a> <span class="age"><a href="item?id=22799263">6 hours ago</a></span> <span
                      id="unv_22799263"></span> | <a href="hide?id=22799263&amp;goto=news">hide</a> | <a
                      href="item?id=22799263">72&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22796038'>
                  <td align="right" valign="top" class="title"><span class="rank">28.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22796038' href='vote?id=22796038&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://github.com/dcf21/star-charter" class="storylink">StarCharter – CLI tool
                      for producing vector-graphics charts of the night sky</a><span class="sitebit comhead"> (<a
                        href="from?site=github.com"><span class="sitestr">github.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22796038">136 points</span> by <a href="user?id=app4soft"
                      class="hnuser">app4soft</a> <span class="age"><a href="item?id=22796038">13 hours ago</a></span> <span
                      id="unv_22796038"></span> | <a href="hide?id=22796038&amp;goto=news">hide</a> | <a
                      href="item?id=22796038">5&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22794533'>
                  <td align="right" valign="top" class="title"><span class="rank">29.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22794533' href='vote?id=22794533&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a
                      href="https://github.com/blanchette/logical_verification_2020/raw/master/hitchhikers_guide.pdf"
                      class="storylink">Lean Book: The Hitchhiker's Guide to Logical Verification [pdf]</a><span
                      class="sitebit comhead"> (<a href="from?site=github.com"><span
                          class="sitestr">github.com</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22794533">131 points</span> by <a href="user?id=kevinbuzzard"
                      class="hnuser">kevinbuzzard</a> <span class="age"><a href="item?id=22794533">15 hours ago</a></span>
                    <span id="unv_22794533"></span> | <a href="hide?id=22794533&amp;goto=news">hide</a> | <a
                      href="item?id=22794533">16&nbsp;comments</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class='athing' id='22794924'>
                  <td align="right" valign="top" class="title"><span class="rank">30.</span></td>
                  <td valign="top" class="votelinks">
                    <center><a id='up_22794924' href='vote?id=22794924&amp;how=up&amp;goto=news'>
                        <div class='votearrow' title='upvote'></div>
                      </a></center>
                  </td>
                  <td class="title"><a href="https://fs.blog/2020/04/conjunctive-events-bias/" class="storylink">Unlikely
                      Optimism: The Conjunctive Events Bias</a><span class="sitebit comhead"> (<a
                        href="from?site=fs.blog"><span class="sitestr">fs.blog</span></a>)</span></td>
                </tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="subtext">
                    <span class="score" id="score_22794924">18 points</span> by <a href="user?id=yarapavan"
                      class="hnuser">yarapavan</a> <span class="age"><a href="item?id=22794924">4 hours ago</a></span> <span
                      id="unv_22794924"></span> | <a href="hide?id=22794924&amp;goto=news">hide</a> | <a
                      href="item?id=22794924">discuss</a>
                  </td>
                </tr>
                <tr class="spacer" style="height:5px"></tr>
                <tr class="morespace" style="height:10px"></tr>
                <tr>
                  <td colspan="2"></td>
                  <td class="title"><a href="news?p=2" class="morelink" rel="next">More</a></td>
                </tr>
              </table>
            </td>
          </tr>
          <tr>
            <td><img src="s.gif" height="10" width="0">
              <table width="100%" cellspacing="0" cellpadding="1">
                <tr>
                  <td bgcolor="ff6600"></td>
                </tr>
              </table><br>
              <center><span class="yclinks"><a href="newsguidelines.html">Guidelines</a>
                  | <a href="newsfaq.html">FAQ</a>
                  | <a href="mailto:hn@ycombinator.com">Support</a>
                  | <a href="https://github.com/HackerNews/API">API</a>
                  | <a href="security.html">Security</a>
                  | <a href="lists">Lists</a>
                  | <a href="bookmarklet.html" rel="nofollow">Bookmarklet</a>
                  | <a href="http://www.ycombinator.com/legal/">Legal</a>
                  | <a href="http://www.ycombinator.com/apply/">Apply to YC</a>
                  | <a href="mailto:hn@ycombinator.com">Contact</a></span><br><br>
                <form method="get" action="//hn.algolia.com/">Search:
                  <input type="text" name="q" value="" size="17" autocorrect="off" spellcheck="false" autocapitalize="off"
                    autocomplete="false">
                </form>
              </center>
            </td>
          </tr>
        </table>
      </center>
    </body>
    <script type='text/javascript' src='hn.js?EdFA4gW5lET4IyyrLeIo'></script>
    
    </html>
    "#;
  html.to_string()
}
