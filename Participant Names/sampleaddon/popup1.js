function open(url)
{
	var current = browser.tabs.getcurrent({
    url:url
  });
}
function onGot(tabInfo) {
  console.log(tabInfo);
}

function onError(error) {
  console.log(`Error: ${error}`);
}

var Current = browser.tabs.getCurrent();
gettingCurrent.then(onGot, onError);