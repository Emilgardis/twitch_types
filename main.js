"use strict";if(!String.prototype.startsWith){String.prototype.startsWith=function(searchString,position){position=position||0;return this.indexOf(searchString,position)===position}}if(!String.prototype.endsWith){String.prototype.endsWith=function(suffix,length){const l=length||this.length;return this.indexOf(suffix,l-suffix.length)!==-1}}if(!DOMTokenList.prototype.add){DOMTokenList.prototype.add=function(className){if(className&&!hasClass(this,className)){if(this.className&&this.className.length>0){this.className+=" "+className}else{this.className=className}}}}if(!DOMTokenList.prototype.remove){DOMTokenList.prototype.remove=function(className){if(className&&this.className){this.className=(" "+this.className+" ").replace(" "+className+" "," ").trim()}}}function getVar(name){const el=document.getElementById("rustdoc-vars");if(el){return el.attributes["data-"+name].value}else{return null}}function resourcePath(basename,extension){return getVar("root-path")+basename+getVar("resource-suffix")+extension}function hideMain(){addClass(document.getElementById(MAIN_ID),"hidden")}function showMain(){removeClass(document.getElementById(MAIN_ID),"hidden")}(function(){window.rootPath=getVar("root-path");window.currentCrate=getVar("current-crate");window.searchJS=resourcePath("search",".js");window.searchIndexJS=resourcePath("search-index",".js");window.settingsJS=resourcePath("settings",".js");const sidebarVars=document.getElementById("sidebar-vars");if(sidebarVars){window.sidebarCurrent={name:sidebarVars.attributes["data-name"].value,ty:sidebarVars.attributes["data-ty"].value,relpath:sidebarVars.attributes["data-relpath"].value,};const mobileLocationTitle=document.querySelector(".mobile-topbar h2.location");const locationTitle=document.querySelector(".sidebar h2.location");if(mobileLocationTitle&&locationTitle){mobileLocationTitle.innerHTML=locationTitle.innerHTML}}}());function getVirtualKey(ev){if("key"in ev&&typeof ev.key!="undefined"){return ev.key}const c=ev.charCode||ev.keyCode;if(c==27){return"Escape"}return String.fromCharCode(c)}const THEME_PICKER_ELEMENT_ID="theme-picker";const THEMES_ELEMENT_ID="theme-choices";const MAIN_ID="main-content";const SETTINGS_BUTTON_ID="settings-menu";const ALTERNATIVE_DISPLAY_ID="alternative-display";const NOT_DISPLAYED_ID="not-displayed";function getThemesElement(){return document.getElementById(THEMES_ELEMENT_ID)}function getThemePickerElement(){return document.getElementById(THEME_PICKER_ELEMENT_ID)}function getSettingsButton(){return document.getElementById(SETTINGS_BUTTON_ID)}function getNakedUrl(){return window.location.href.split("?")[0].split("#")[0]}function showThemeButtonState(){const themePicker=getThemePickerElement();const themeChoices=getThemesElement();themeChoices.style.display="block";themePicker.style.borderBottomRightRadius="0";themePicker.style.borderBottomLeftRadius="0"}function hideThemeButtonState(){const themePicker=getThemePickerElement();const themeChoices=getThemesElement();themeChoices.style.display="none";themePicker.style.borderBottomRightRadius="3px";themePicker.style.borderBottomLeftRadius="3px"}window.hideSettings=()=>{};(function(){if(!document.location.href.startsWith("file:///")){return}const themeChoices=getThemesElement();const themePicker=getThemePickerElement();const availableThemes=getVar("themes").split(",");removeClass(themeChoices.parentElement,"hidden");function switchThemeButtonState(){if(themeChoices.style.display==="block"){hideThemeButtonState()}else{showThemeButtonState()}}function handleThemeButtonsBlur(e){const active=document.activeElement;const related=e.relatedTarget;if(active.id!==THEME_PICKER_ELEMENT_ID&&(!active.parentNode||active.parentNode.id!==THEMES_ELEMENT_ID)&&(!related||(related.id!==THEME_PICKER_ELEMENT_ID&&(!related.parentNode||related.parentNode.id!==THEMES_ELEMENT_ID)))){hideThemeButtonState()}}themePicker.onclick=switchThemeButtonState;themePicker.onblur=handleThemeButtonsBlur;availableThemes.forEach(item=>{const but=document.createElement("button");but.textContent=item;but.onclick=()=>{switchTheme(window.currentTheme,window.mainTheme,item,true);useSystemTheme(false)};but.onblur=handleThemeButtonsBlur;themeChoices.appendChild(but)})}());function insertAfter(newNode,referenceNode){referenceNode.parentNode.insertBefore(newNode,referenceNode.nextSibling)}function getOrCreateSection(id,classes){let el=document.getElementById(id);if(!el){el=document.createElement("section");el.id=id;el.className=classes;insertAfter(el,document.getElementById(MAIN_ID))}return el}function getAlternativeDisplayElem(){return getOrCreateSection(ALTERNATIVE_DISPLAY_ID,"content hidden")}function getNotDisplayedElem(){return getOrCreateSection(NOT_DISPLAYED_ID,"hidden")}function switchDisplayedElement(elemToDisplay){const el=getAlternativeDisplayElem();if(el.children.length>0){getNotDisplayedElem().appendChild(el.firstElementChild)}if(elemToDisplay===null){addClass(el,"hidden");showMain();return}el.appendChild(elemToDisplay);hideMain();removeClass(el,"hidden")}function browserSupportsHistoryApi(){return window.history&&typeof window.history.pushState==="function"}function loadCss(cssFileName){const link=document.createElement("link");link.href=resourcePath(cssFileName,".css");link.type="text/css";link.rel="stylesheet";document.getElementsByTagName("head")[0].appendChild(link)}(function(){function loadScript(url){const script=document.createElement('script');script.src=url;document.head.append(script)}getSettingsButton().onclick=event=>{addClass(getSettingsButton(),"rotate");event.preventDefault();loadCss("settings");loadScript(window.settingsJS)};window.searchState={loadingText:"Loading search results...",input:document.getElementsByClassName("search-input")[0],outputElement:()=>{let el=document.getElementById("search");if(!el){el=document.createElement("section");el.id="search";getNotDisplayedElem().appendChild(el)}return el},title:document.title,titleBeforeSearch:document.title,timeout:null,currentTab:0,focusedByTab:[null,null,null],clearInputTimeout:()=>{if(searchState.timeout!==null){clearTimeout(searchState.timeout);searchState.timeout=null}},isDisplayed:()=>searchState.outputElement().parentElement.id===ALTERNATIVE_DISPLAY_ID,focus:()=>{searchState.input.focus()},defocus:()=>{searchState.input.blur()},showResults:search=>{if(search===null||typeof search==='undefined'){search=searchState.outputElement()}switchDisplayedElement(search);searchState.mouseMovedAfterSearch=false;document.title=searchState.title},hideResults:()=>{switchDisplayedElement(null);document.title=searchState.titleBeforeSearch;if(browserSupportsHistoryApi()){history.replaceState(null,window.currentCrate+" - Rust",getNakedUrl()+window.location.hash)}},getQueryStringParams:()=>{const params={};window.location.search.substring(1).split("&").map(s=>{const pair=s.split("=");params[decodeURIComponent(pair[0])]=typeof pair[1]==="undefined"?null:decodeURIComponent(pair[1])});return params},setup:()=>{const search_input=searchState.input;if(!searchState.input){return}let searchLoaded=false;function loadSearch(){if(!searchLoaded){searchLoaded=true;loadScript(window.searchJS);loadScript(window.searchIndexJS)}}search_input.addEventListener("focus",()=>{search_input.origPlaceholder=search_input.placeholder;search_input.placeholder="Type your search here.";loadSearch()});if(search_input.value!==''){loadSearch()}const params=searchState.getQueryStringParams();if(params.search!==undefined){const search=searchState.outputElement();search.innerHTML="<h3 class=\"search-loading\">"+searchState.loadingText+"</h3>";searchState.showResults(search);loadSearch()}},};function getPageId(){if(window.location.hash){const tmp=window.location.hash.replace(/^#/,"");if(tmp.length>0){return tmp}}return null}const toggleAllDocsId="toggle-all-docs";let savedHash="";function handleHashes(ev){if(ev!==null&&searchState.isDisplayed()&&ev.newURL){switchDisplayedElement(null);const hash=ev.newURL.slice(ev.newURL.indexOf("#")+1);if(browserSupportsHistoryApi()){history.replaceState(null,"",getNakedUrl()+window.location.search+"#"+hash)}const elem=document.getElementById(hash);if(elem){elem.scrollIntoView()}}if(savedHash!==window.location.hash){savedHash=window.location.hash;if(savedHash.length===0){return}expandSection(savedHash.slice(1))}}function onHashChange(ev){const sidebar=document.getElementsByClassName("sidebar")[0];removeClass(sidebar,"shown");handleHashes(ev)}function openParentDetails(elem){while(elem){if(elem.tagName==="DETAILS"){elem.open=true}elem=elem.parentNode}}function expandSection(id){openParentDetails(document.getElementById(id))}function getHelpElement(build){if(build){buildHelperPopup()}return document.getElementById("help")}function displayHelp(display,ev,help){if(display){help=help?help:getHelpElement(true);if(hasClass(help,"hidden")){ev.preventDefault();removeClass(help,"hidden");addClass(document.body,"blur")}}else{help=help?help:getHelpElement(false);if(help&&!hasClass(help,"hidden")){ev.preventDefault();addClass(help,"hidden");removeClass(document.body,"blur")}}}function handleEscape(ev){searchState.clearInputTimeout();const help=getHelpElement(false);if(help&&!hasClass(help,"hidden")){displayHelp(false,ev,help)}else{switchDisplayedElement(null);if(browserSupportsHistoryApi()){history.replaceState(null,window.currentCrate+" - Rust",getNakedUrl()+window.location.hash)}ev.preventDefault()}searchState.defocus();hideThemeButtonState()}const disableShortcuts=getSettingValue("disable-shortcuts")==="true";function handleShortcut(ev){if(ev.ctrlKey||ev.altKey||ev.metaKey||disableShortcuts){return}let themePicker;if(document.activeElement.tagName==="INPUT"){switch(getVirtualKey(ev)){case"Escape":handleEscape(ev);break}}else{switch(getVirtualKey(ev)){case"Escape":handleEscape(ev);break;case"s":case"S":displayHelp(false,ev);ev.preventDefault();searchState.focus();break;case"+":case"-":ev.preventDefault();toggleAllDocs();break;case"?":displayHelp(true,ev);break;case"t":case"T":displayHelp(false,ev);ev.preventDefault();themePicker=getThemePickerElement();themePicker.click();themePicker.focus();break;default:if(getThemePickerElement().parentNode.contains(ev.target)){handleThemeKeyDown(ev)}}}}function handleThemeKeyDown(ev){const active=document.activeElement;const themes=getThemesElement();switch(getVirtualKey(ev)){case"ArrowUp":ev.preventDefault();if(active.previousElementSibling&&ev.target.id!==THEME_PICKER_ELEMENT_ID){active.previousElementSibling.focus()}else{showThemeButtonState();themes.lastElementChild.focus()}break;case"ArrowDown":ev.preventDefault();if(active.nextElementSibling&&ev.target.id!==THEME_PICKER_ELEMENT_ID){active.nextElementSibling.focus()}else{showThemeButtonState();themes.firstElementChild.focus()}break;case"Enter":case"Return":case"Space":if(ev.target.id===THEME_PICKER_ELEMENT_ID&&themes.style.display==="none"){ev.preventDefault();showThemeButtonState();themes.firstElementChild.focus()}break;case"Home":ev.preventDefault();themes.firstElementChild.focus();break;case"End":ev.preventDefault();themes.lastElementChild.focus();break}}document.addEventListener("keypress",handleShortcut);document.addEventListener("keydown",handleShortcut);window.initSidebarItems=items=>{const sidebar=document.getElementsByClassName("sidebar-elems")[0];let others;const current=window.sidebarCurrent;function addSidebarCrates(crates){if(!hasClass(document.body,"crate")){return}const div=document.createElement("div");div.className="block crate";div.innerHTML="<h3>Crates</h3>";const ul=document.createElement("ul");div.appendChild(ul);for(const crate of crates){let klass="crate";if(window.rootPath!=="./"&&crate===window.currentCrate){klass+=" current"}const link=document.createElement("a");link.href=window.rootPath+crate+"/index.html";link.className=klass;link.textContent=crate;const li=document.createElement("li");li.appendChild(link);ul.appendChild(li)}others.appendChild(div)}function block(shortty,id,longty){const filtered=items[shortty];if(!filtered){return}const div=document.createElement("div");div.className="block "+shortty;const h3=document.createElement("h3");h3.innerHTML=`<a href="index.html#${id}">${longty}</a>`;div.appendChild(h3);const ul=document.createElement("ul");for(const item of filtered){const name=item[0];const desc=item[1];let klass=shortty;if(name===current.name&&shortty===current.ty){klass+=" current"}let path;if(shortty==="mod"){path=name+"/index.html"}else{path=shortty+"."+name+".html"}const link=document.createElement("a");link.href=current.relpath+path;link.title=desc;link.className=klass;link.textContent=name;const li=document.createElement("li");li.appendChild(link);ul.appendChild(li)}div.appendChild(ul);others.appendChild(div)}if(sidebar){others=document.createElement("div");others.className="others";sidebar.appendChild(others);const isModule=hasClass(document.body,"mod");if(!isModule){block("primitive","primitives","Primitive Types");block("mod","modules","Modules");block("macro","macros","Macros");block("struct","structs","Structs");block("enum","enums","Enums");block("union","unions","Unions");block("constant","constants","Constants");block("static","static","Statics");block("trait","traits","Traits");block("fn","functions","Functions");block("type","types","Type Definitions");block("foreigntype","foreign-types","Foreign Types");block("keyword","keywords","Keywords");block("traitalias","trait-aliases","Trait Aliases")}addSidebarCrates(window.ALL_CRATES)}};window.register_implementors=imp=>{const implementors=document.getElementById("implementors-list");const synthetic_implementors=document.getElementById("synthetic-implementors-list");const inlined_types=new Set();if(synthetic_implementors){onEachLazy(synthetic_implementors.getElementsByClassName("impl"),el=>{const aliases=el.getAttribute("data-aliases");if(!aliases){return}aliases.split(",").forEach(alias=>{inlined_types.add(alias)})})}let currentNbImpls=implementors.getElementsByClassName("impl").length;const traitName=document.querySelector("h1.fqn > .in-band > .trait").textContent;const baseIdName="impl-"+traitName+"-";const libs=Object.getOwnPropertyNames(imp);const ignoreExternCrates=document.querySelector("script[data-ignore-extern-crates]").getAttribute("data-ignore-extern-crates");for(const lib of libs){if(lib===window.currentCrate||ignoreExternCrates.indexOf(lib)!==-1){continue}const structs=imp[lib];struct_loop:for(const struct of structs){const list=struct.synthetic?synthetic_implementors:implementors;if(struct.synthetic){for(const struct_type of struct.types){if(inlined_types.has(struct_type)){continue struct_loop}inlined_types.add(struct_type)}}const code=document.createElement("h3");code.innerHTML=struct.text;addClass(code,"code-header");addClass(code,"in-band");onEachLazy(code.getElementsByTagName("a"),elem=>{const href=elem.getAttribute("href");if(href&&href.indexOf("http")!==0){elem.setAttribute("href",window.rootPath+href)}});const currentId=baseIdName+currentNbImpls;const anchor=document.createElement("a");anchor.href="#"+currentId;addClass(anchor,"anchor");const display=document.createElement("div");display.id=currentId;addClass(display,"impl");display.appendChild(anchor);display.appendChild(code);list.appendChild(display);currentNbImpls+=1}}};if(window.pending_implementors){window.register_implementors(window.pending_implementors)}function labelForToggleButton(sectionIsCollapsed){if(sectionIsCollapsed){return"+"}return"\u2212"}function toggleAllDocs(){const innerToggle=document.getElementById(toggleAllDocsId);if(!innerToggle){return}let sectionIsCollapsed=false;if(hasClass(innerToggle,"will-expand")){removeClass(innerToggle,"will-expand");onEachLazy(document.getElementsByClassName("rustdoc-toggle"),e=>{if(!hasClass(e,"type-contents-toggle")){e.open=true}});innerToggle.title="collapse all docs"}else{addClass(innerToggle,"will-expand");onEachLazy(document.getElementsByClassName("rustdoc-toggle"),e=>{if(e.parentNode.id!=="implementations-list"||(!hasClass(e,"implementors-toggle")&&!hasClass(e,"type-contents-toggle"))){e.open=false}});sectionIsCollapsed=true;innerToggle.title="expand all docs"}innerToggle.children[0].innerText=labelForToggleButton(sectionIsCollapsed)}(function(){const toggles=document.getElementById(toggleAllDocsId);if(toggles){toggles.onclick=toggleAllDocs}const hideMethodDocs=getSettingValue("auto-hide-method-docs")==="true";const hideImplementations=getSettingValue("auto-hide-trait-implementations")==="true";const hideLargeItemContents=getSettingValue("auto-hide-large-items")!=="false";function setImplementorsTogglesOpen(id,open){const list=document.getElementById(id);if(list!==null){onEachLazy(list.getElementsByClassName("implementors-toggle"),e=>{e.open=open})}}if(hideImplementations){setImplementorsTogglesOpen("trait-implementations-list",false);setImplementorsTogglesOpen("blanket-implementations-list",false)}onEachLazy(document.getElementsByClassName("rustdoc-toggle"),e=>{if(!hideLargeItemContents&&hasClass(e,"type-contents-toggle")){e.open=true}if(hideMethodDocs&&hasClass(e,"method-toggle")){e.open=false}});const pageId=getPageId();if(pageId!==null){expandSection(pageId)}}());(function(){let lineNumbersFunc=()=>{};if(getSettingValue("line-numbers")==="true"){lineNumbersFunc=x=>{const count=x.textContent.split("\n").length;const elems=[];for(let i=0;i<count;++i){elems.push(i+1)}const node=document.createElement("pre");addClass(node,"line-number");node.innerHTML=elems.join("\n");x.parentNode.insertBefore(node,x)}}onEachLazy(document.getElementsByClassName("rust-example-rendered"),e=>{if(hasClass(e,"compile_fail")){e.addEventListener("mouseover",function(){this.parentElement.previousElementSibling.childNodes[0].style.color="#f00"});e.addEventListener("mouseout",function(){this.parentElement.previousElementSibling.childNodes[0].style.color=""})}else if(hasClass(e,"ignore")){e.addEventListener("mouseover",function(){this.parentElement.previousElementSibling.childNodes[0].style.color="#ff9200"});e.addEventListener("mouseout",function(){this.parentElement.previousElementSibling.childNodes[0].style.color=""})}lineNumbersFunc(e)})}());function hideSidebar(){const sidebar=document.getElementsByClassName("sidebar")[0];removeClass(sidebar,"shown")}function handleClick(id,f){const elem=document.getElementById(id);if(elem){elem.addEventListener("click",f)}}handleClick("help-button",ev=>{displayHelp(true,ev)});handleClick(MAIN_ID,()=>{hideSidebar()});onEachLazy(document.getElementsByTagName("a"),el=>{if(el.hash){el.addEventListener("click",()=>{expandSection(el.hash.slice(1));hideSidebar()})}});onEachLazy(document.querySelectorAll(".rustdoc-toggle > summary:not(.hideme)"),el=>{el.addEventListener("click",e=>{if(e.target.tagName!=="SUMMARY"&&e.target.tagName!=="A"){e.preventDefault()}})});onEachLazy(document.getElementsByClassName("notable-traits"),e=>{e.onclick=function(){this.getElementsByClassName('notable-traits-tooltiptext')[0].classList.toggle("force-tooltip")}});const sidebar_menu_toggle=document.getElementsByClassName("sidebar-menu-toggle")[0];if(sidebar_menu_toggle){sidebar_menu_toggle.addEventListener("click",()=>{const sidebar=document.getElementsByClassName("sidebar")[0];if(!hasClass(sidebar,"shown")){addClass(sidebar,"shown")}else{removeClass(sidebar,"shown")}})}let buildHelperPopup=()=>{const popup=document.createElement("aside");addClass(popup,"hidden");popup.id="help";popup.addEventListener("click",ev=>{if(ev.target===popup){displayHelp(false,ev)}});const book_info=document.createElement("span");book_info.className="top";book_info.innerHTML="You can find more information in \
            <a href=\"https://doc.rust-lang.org/rustdoc/\">the rustdoc book</a>.";const container=document.createElement("div");const shortcuts=[["?","Show this help dialog"],["S","Focus the search field"],["T","Focus the theme picker menu"],["↑","Move up in search results"],["↓","Move down in search results"],["← / →","Switch result tab (when results focused)"],["&#9166;","Go to active search result"],["+","Expand all sections"],["-","Collapse all sections"],].map(x=>"<dt>"+x[0].split(" ").map((y,index)=>(index&1)===0?"<kbd>"+y+"</kbd>":" "+y+" ").join("")+"</dt><dd>"+x[1]+"</dd>").join("");const div_shortcuts=document.createElement("div");addClass(div_shortcuts,"shortcuts");div_shortcuts.innerHTML="<h2>Keyboard Shortcuts</h2><dl>"+shortcuts+"</dl></div>";const infos=["Prefix searches with a type followed by a colon (e.g., <code>fn:</code>) to \
             restrict the search to a given item kind.","Accepted kinds are: <code>fn</code>, <code>mod</code>, <code>struct</code>, \
             <code>enum</code>, <code>trait</code>, <code>type</code>, <code>macro</code>, \
             and <code>const</code>.","Search functions by type signature (e.g., <code>vec -&gt; usize</code> or \
             <code>* -&gt; vec</code>)","Search multiple things at once by splitting your query with comma (e.g., \
             <code>str,u8</code> or <code>String,struct:Vec,test</code>)","You can look for items with an exact name by putting double quotes around \
             your request: <code>\"string\"</code>","Look for items inside another one by searching for a path: <code>vec::Vec</code>",].map(x=>"<p>"+x+"</p>").join("");const div_infos=document.createElement("div");addClass(div_infos,"infos");div_infos.innerHTML="<h2>Search Tricks</h2>"+infos;container.appendChild(book_info);container.appendChild(div_shortcuts);container.appendChild(div_infos);const rustdoc_version=document.createElement("span");rustdoc_version.className="bottom";const rustdoc_version_code=document.createElement("code");rustdoc_version_code.innerText="rustdoc "+getVar("rustdoc-version");rustdoc_version.appendChild(rustdoc_version_code);container.appendChild(rustdoc_version);popup.appendChild(container);insertAfter(popup,document.querySelector("main"));buildHelperPopup=()=>{}};onHashChange(null);window.addEventListener("hashchange",onHashChange);searchState.setup()}());(function(){let reset_button_timeout=null;window.copy_path=but=>{const parent=but.parentElement;const path=[];onEach(parent.childNodes,child=>{if(child.tagName==='A'){path.push(child.textContent)}});const el=document.createElement('textarea');el.value=path.join('::');el.setAttribute('readonly','');el.style.position='absolute';el.style.left='-9999px';document.body.appendChild(el);el.select();document.execCommand('copy');document.body.removeChild(el);but.children[0].style.display='none';let tmp;if(but.childNodes.length<2){tmp=document.createTextNode('✓');but.appendChild(tmp)}else{onEachLazy(but.childNodes,e=>{if(e.nodeType===Node.TEXT_NODE){tmp=e;return true}});tmp.textContent='✓'}if(reset_button_timeout!==null){window.clearTimeout(reset_button_timeout)}function reset_button(){tmp.textContent='';reset_button_timeout=null;but.children[0].style.display=""}reset_button_timeout=window.setTimeout(reset_button,1000)}}())