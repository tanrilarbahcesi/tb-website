function getBlogName() {
	const urlParams = new URLSearchParams(window.location.search);
	return urlParams.get('sayfa') || 'main.html'; // URL'den blog parametresi, yoksa varsayılan
}

function updateMetaTags(meta) {
	document.title = meta.title;

	let descriptionMeta = document.querySelector('meta[name="description"]');
	descriptionMeta.content = meta.description;

	let keywordsMeta = document.querySelector('meta[name="keywords"]');
	keywordsMeta.content = meta.keywords;
}

let blogName = getBlogName();
if (blogName == "main.html") {
    this.document.getElementById("content").classList.remove("content");
}
else {
    blogName +=".html";
}

fetch(blogName)
.then(response => {
    if (!response.ok) {
        throw new Error('Sayfa bulunamadı!');
    }
    return response.text();
})
.then(content => {
    document.getElementById('content').innerHTML = content;
    const metaTags = document.getElementById('blog-meta');
    if (metaTags) {
    	const meta = JSON.parse(metaTags.textContent);
    	updateMetaTags(meta);
    }
})
.catch(error => {
    document.getElementById('content').innerHTML = '<h2>Sayfa bulunamadı.</h2>';
});
