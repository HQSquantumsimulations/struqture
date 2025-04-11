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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded "><a href="physical_types/intro.html"><strong aria-hidden="true">1.</strong> How to use struqture</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="physical_types/spins/intro.html"><strong aria-hidden="true">1.1.</strong> Spins</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="physical_types/spins/products.html"><strong aria-hidden="true">1.1.1.</strong> Building blocks: PauliProduct &amp; DecoherenceProduct</a></li><li class="chapter-item expanded "><a href="physical_types/spins/noisefree.html"><strong aria-hidden="true">1.1.2.</strong> Coherent dynamics: PauliOperator &amp; PauliHamiltonian</a></li><li class="chapter-item expanded "><a href="physical_types/spins/noisy.html"><strong aria-hidden="true">1.1.3.</strong> Decoherent dynamics: PauliLindbladNoiseOperator &amp; PauliLindbladOpenSystem</a></li><li class="chapter-item expanded "><a href="physical_types/spins/plus_minus.html"><strong aria-hidden="true">1.1.4.</strong> Alternative basis: {+, -, z}</a></li></ol></li><li class="chapter-item expanded "><a href="physical_types/bosons/intro.html"><strong aria-hidden="true">1.2.</strong> Bosons</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="physical_types/bosons/products.html"><strong aria-hidden="true">1.2.1.</strong> Building blocks: BosonProduct &amp; HermitianBosonProduct</a></li><li class="chapter-item expanded "><a href="physical_types/bosons/noisefree.html"><strong aria-hidden="true">1.2.2.</strong> Coherent dynamics: BosonOperator &amp; BosonHamiltonian</a></li><li class="chapter-item expanded "><a href="physical_types/bosons/noisy.html"><strong aria-hidden="true">1.2.3.</strong> Decoherent dynamics: BosonLindbladNoiseOperator &amp; BosonLindbladOpenSystem</a></li></ol></li><li class="chapter-item expanded "><a href="physical_types/fermions/intro.html"><strong aria-hidden="true">1.3.</strong> Fermions</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="physical_types/fermions/products.html"><strong aria-hidden="true">1.3.1.</strong> Building blocks: FermionProduct &amp; HermitianFermionProduct</a></li><li class="chapter-item expanded "><a href="physical_types/fermions/noisefree.html"><strong aria-hidden="true">1.3.2.</strong> Coherent dynamics: FermionOperator &amp; FermionHamiltonian</a></li><li class="chapter-item expanded "><a href="physical_types/fermions/noisy.html"><strong aria-hidden="true">1.3.3.</strong> Decoherent dynamics: FermionLindbladNoiseOperator &amp; FermionLindbladOpenSystem</a></li></ol></li><li class="chapter-item expanded "><a href="physical_types/mixed_systems/intro.html"><strong aria-hidden="true">1.4.</strong> Mixed systems</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="physical_types/mixed_systems/products.html"><strong aria-hidden="true">1.4.1.</strong> Building blocks: MixedProduct &amp; HermitianMixedProduct &amp; MixedDecoherenceProducts</a></li><li class="chapter-item expanded "><a href="physical_types/mixed_systems/noisefree.html"><strong aria-hidden="true">1.4.2.</strong> Coherent dynamics: MixedOperator &amp; MixedHamiltonian</a></li><li class="chapter-item expanded "><a href="physical_types/mixed_systems/noisy.html"><strong aria-hidden="true">1.4.3.</strong> Decoherent dynamics: MixedLindbladNoiseOperator &amp; MixedLindbladOpenSystem</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="container_types/intro.html"><strong aria-hidden="true">2.</strong> Struqture: design &amp; implementation</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="container_types/indices.html"><strong aria-hidden="true">2.1.</strong> Indices and Products</a></li><li class="chapter-item expanded "><a href="container_types/operators_hamiltonians.html"><strong aria-hidden="true">2.2.</strong> Operators and Hamiltonians</a></li><li class="chapter-item expanded "><a href="container_types/noise_operators.html"><strong aria-hidden="true">2.3.</strong> Lindblad Noise Operators</a></li><li class="chapter-item expanded "><a href="container_types/open_systems.html"><strong aria-hidden="true">2.4.</strong> Lindblad Open Systems</a></li></ol></li><li class="chapter-item expanded "><a href="example.html"><strong aria-hidden="true">3.</strong> Applied Example</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
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
