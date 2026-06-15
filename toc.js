// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="introduction/index.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="cli/index.html"><strong aria-hidden="true">2.</strong> CLI Tools</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="cli/document/index.html"><strong aria-hidden="true">2.1.</strong> Document</a></li><li class="chapter-item expanded "><a href="cli/draw/index.html"><strong aria-hidden="true">2.2.</strong> Draw</a></li></ol></li><li class="chapter-item expanded "><a href="ui/index.html"><strong aria-hidden="true">3.</strong> UI Tools</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="ui/editor/index.html"><strong aria-hidden="true">3.1.</strong> Editor</a></li><li class="chapter-item expanded "><a href="ui/navigator/index.html"><strong aria-hidden="true">3.2.</strong> Navigator</a></li></ol></li><li class="chapter-item expanded "><a href="nocturn/introduction/index.html"><strong aria-hidden="true">4.</strong> Nocturn: Visual Language</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="nocturn/ontologies/index.html"><strong aria-hidden="true">4.1.</strong> Ontologies</a></li><li class="chapter-item expanded "><a href="nocturn/nodes/index.html"><strong aria-hidden="true">4.2.</strong> Nodes</a></li><li class="chapter-item expanded "><a href="nocturn/class-assertions/index.html"><strong aria-hidden="true">4.3.</strong> Class Assertions</a></li><li class="chapter-item expanded "><a href="nocturn/object-properties/index.html"><strong aria-hidden="true">4.4.</strong> Object Properties</a></li><li class="chapter-item expanded "><a href="nocturn/datatype-assertions/index.html"><strong aria-hidden="true">4.5.</strong> Datatype Assertions</a></li><li class="chapter-item expanded "><a href="nocturn/datatype-properties/index.html"><strong aria-hidden="true">4.6.</strong> Datatype Properties</a></li><li class="chapter-item expanded "><a href="nocturn/annotation-properties/index.html"><strong aria-hidden="true">4.7.</strong> Annotation Properties</a></li><li class="chapter-item expanded "><a href="nocturn/individual-relations/index.html"><strong aria-hidden="true">4.8.</strong> Individual Assertions</a></li><li class="chapter-item expanded "><a href="nocturn/extended-property-notation/index.html"><strong aria-hidden="true">4.9.</strong> Extended Property Notation</a></li><li class="chapter-item expanded "><a href="nocturn/expressions/index.html"><strong aria-hidden="true">4.10.</strong> Class and Value Expressions</a></li><li class="chapter-item expanded "><a href="nocturn/general-axioms/index.html"><strong aria-hidden="true">4.11.</strong> General Axioms</a></li><li class="chapter-item expanded "><a href="nocturn/example-wine/index.html"><strong aria-hidden="true">4.12.</strong> Appendix: Wine Ontology</a></li><li class="chapter-item expanded "><a href="nocturn/graphviz/index.html"><strong aria-hidden="true">4.13.</strong> Appendix: GraphViz</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
