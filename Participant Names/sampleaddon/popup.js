/*
Note: underdevolopement

function onget(tab) {
  //Your code below...
  var tabUrl = encodeURIComponent(tab.url);
  var tabTitle = encodeURIComponent(tab.title);

  //Update the url here.
  browser.tabs.update(tab.id, {url: url});
}
*/

function open(url)
{
  //browser.tabs.getCurrent().then(onget);?
  browser.tabs.create({url:url});
}

document.addEventListener("click",(e)=>{
if(e.target.id==="a"){
	open("http://www.andhrauniversity.edu.in");
} else if(e.target.id==="b"){
open("http://www.pragati.ac.in");
}else if(e.target=="c"){
	open("https://www.aec.edu.in/");
}
else if(e.target.id=="d"){
	open("http://www.jntuk.edu.in");
}
else if(e.target.id=="e"){
	open("http://www.vishnu.edu.in");
}
else if(e.target.id="f"){
	open("http://www.srkrec.ac.in");
}

});
