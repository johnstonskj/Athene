
// This executes early, preparing highlight.js configurations
document.addEventListener("DOMContentLoaded", () => {
    if (typeof hljs !== 'undefined') {
        console.log("Registering custom Highlight.js languages...");
        hljs.registerLanguage("owl", hljsOwl);
        hljs.registerLanguage("sparql", hljsSparql);
        hljs.registerLanguage("turtle", hljsTurtle);
    } else {
        console.log("Suspicious that there's no 'hljs' defined...");
    }
});