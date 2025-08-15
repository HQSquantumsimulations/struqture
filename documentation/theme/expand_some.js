// Expand only selected sections in mdBook's sidebar.
// Place this file at: theme/expand_some.js
console.log("[mdBook] expand_some.js loaded");

(function () {
  // Use hrefs so titles/whitespace don’t matter. These are from your SUMMARY.md
  // and mdBook's default output paths.
  const targets_to_expand = [
    "introduction.html",
    "physical_types/intro.html",         // How to use struqture
    "physical_types/spins/intro.html",   // Spins
    "example.html",                      // Applied Example
  ];

  // Normalize target endings: also accept .../ and .../index.html
  const href_needles = new Set();
  for (const t of targets_to_expand) {
    const base = t.replace(/^\.\//, "").replace(/\/$/, "").toLowerCase();
    href_needles.add(base);
  }

  function normalise_href(href) {
    try {
      // Turn relative into absolute to strip query/hash reliably
      const u = new URL(href, window.location.href);
      let p = (u.pathname || "").toLowerCase();
      // Remove leading base path if any (during mdbook serve this is usually "/")
      // and strip leading slash for consistent "foo/bar.html" form
      p = p.replace(/^\/+/, "");
      // Accept both folder and index.html
      if (p.endsWith("/")) p = p.slice(0, -1);
      return p;
    } catch {
      return (href || "").toLowerCase().replace(/^\.?\//, "").replace(/\/$/, "");
    }
  }

  function matches_target(a) {
    const raw = a.getAttribute("href");
    if (!raw) return false;
    const p = normalise_href(raw);
    if (href_needles.has(p)) return true;
    // Accept ending matches (in case site is hosted under a subpath)
    for (const needle of href_needles) {
      if (p.endsWith(needle)) return true;
    }
    return false;
  }

  function is_expanded(li) {
    const cb = li.querySelector(':scope > input[type="checkbox"], :scope > label > input[type="checkbox"]');
    if (cb) return !!cb.checked;
    const details = li.querySelector(':scope > details');
    if (details) return !!details.open;
    const btn = li.querySelector(':scope > .chapter-item-toggle, :scope > button.toggle, :scope > a.toggle');
    if (btn && btn.getAttribute("aria-expanded")) {
      return btn.getAttribute("aria-expanded") === "true";
    }
    return li.classList.contains("expanded");
  }

  function set_expanded(li, expanded) {
    // details/summary
    const details = li.querySelector(':scope > details');
    if (details) details.open = expanded;

    // checkbox-based (classic mdBook theme)
    const cb = li.querySelector(':scope > input[type="checkbox"], :scope > label > input[type="checkbox"]');
    if (cb && cb.checked !== expanded) {
      cb.checked = expanded;
      cb.dispatchEvent(new Event('change', { bubbles: true }));
    }

    // button/aria-expanded variant (newer themes)
    const btn = li.querySelector(':scope > .chapter-item-toggle, :scope > button.toggle, :scope > a.toggle, :scope > summary');
    if (btn) {
      const want = expanded ? "true" : "false";
      if (btn.getAttribute("aria-expanded") !== want) {
        // click tends to let theme JS update classes/height smoothly
        btn.click?.();
        btn.setAttribute("aria-expanded", want);
      }
    }

    // class fallback
    li.classList.toggle("expanded", expanded);
  }

  function expand_ancestors(li) {
    let cur = li;
    while (cur && cur.matches && cur.matches('li, li.chapter, li.chapter-item, li.section')) {
      if (!is_expanded(cur)) set_expanded(cur, true);
      cur = cur.parentElement?.closest?.('li, li.chapter, li.chapter-item, li.section');
    }
  }

  function expand_from_link(a) {
    const li = a.closest('li');
    if (!li) return false;
    set_expanded(li, true);
    expand_ancestors(li);
    return true;
  }

  function sidebar_el() {
    return document.querySelector('#sidebar, .sidebar, nav[aria-label="Table of contents"]');
  }

  function folding_controls_ready(sidebar) {
    // Run once any known folding control appears
    return !!sidebar.querySelector(
      'input[type="checkbox"], details, .chapter-item-toggle, button.toggle, a.toggle, summary'
    );
  }

  function run_once() {
    const sidebar = sidebar_el();
    if (!sidebar) return false;

    let expanded = 0;
    const links = sidebar.querySelectorAll('a[href]');
    for (const a of links) {
      if (matches_target(a)) {
        if (expand_from_link(a)) expanded++;
      }
    }
    console.log(`[mdBook] expand_some: expanded ${expanded} branch(es).`);
    return expanded > 0;
  }

  // Retry until folding is initialized; also observe later DOM changes (live reload, theme scripts)
  function wait_and_run() {
    const sb = sidebar_el();
    if (!sb) return;

    const try_to_run = () => run_once();

    if (folding_controls_ready(sb)) {
      // Initial run
      try_to_run();
    } else {
      // Poll briefly until controls show up
      let tries = 0;
      const timer = setInterval(() => {
        if (folding_controls_ready(sb) || ++tries > 60) {
          clearInterval(timer);
          try_to_run();
        }
      }, 100);
    }

    // Watch for sidebar mutations and re-apply if needed
    const mo = new MutationObserver(() => {
      // Debounce a little
      clearTimeout(mo._t);
      mo._t = setTimeout(try_to_run, 50);
    });
    mo.observe(sb, { subtree: true, childList: true, attributes: true });
  }

  window.addEventListener('DOMContentLoaded', wait_and_run);
  window.addEventListener('load', wait_and_run);
  window.addEventListener('hashchange', wait_and_run);
})();