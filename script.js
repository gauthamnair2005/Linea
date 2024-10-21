// Script for the website

document.addEventListener('scroll', function() {
    const footer = document.getElementById('footer');
    if (window.scrollY + window.innerHeight >= document.body.scrollHeight) {
        footer.style.display = 'block';
    } else {
        footer.style.display = 'none';
    }
});