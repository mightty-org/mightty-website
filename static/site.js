// The page wears one of Mightty's themes: "Change" in the footer picks one at
// random, a named button picks that one, and the choice is remembered in
// this browser. The themes load the first time they're asked for.
(() => {
  const KEY = "mightty-theme";
  let themes = null;

  const load = async () => {
    if (!themes) {
      const reply = await fetch(`${document.documentElement.dataset.base || ""}/themes.json`);
      themes = await reply.json();
    }
    return themes;
  };

  // A theme's colours: background, foreground, cursor, selection, palette 0-15.
  const apply = (theme) => {
    const root = document.documentElement.style;
    if (!isTheme(theme)) theme = null;
    if (!theme) {
      for (const name of ["--bg", "--fg", "--cursor", "--sel", ...Array.from({ length: 16 }, (_, i) => `--c${i}`)]) {
        root.removeProperty(name);
      }
    } else {
      const [bg, fg, cursor, sel, ...palette] = theme.c;
      root.setProperty("--bg", bg);
      root.setProperty("--fg", fg);
      root.setProperty("--cursor", cursor);
      root.setProperty("--sel", sel);
      palette.forEach((color, i) => root.setProperty(`--c${i}`, color));
    }
    const name = theme ? theme.n : "Mightty";
    for (const el of document.querySelectorAll("[data-theme-name]")) el.textContent = name;
    for (const button of document.querySelectorAll("[data-theme]")) {
      button.setAttribute("aria-pressed", String(button.dataset.theme === name));
    }
    try {
      // The colours themselves, so the next page wears them at once.
      if (theme) localStorage.setItem(KEY, JSON.stringify(theme));
      else localStorage.removeItem(KEY);
    } catch {}
  };

  // Colours go into the page's styles, so only colours are let in: what's
  // saved is shared with every site on this address.
  const isTheme = (t) =>
    t && typeof t.n === "string" && Array.isArray(t.c) && t.c.length === 20 && t.c.every((c) => /^#[0-9a-f]{6}$/i.test(c));

  const byName = async (name) => (await load()).find((t) => t.n === name);

  // Applies a theme the user picked, fading to it.
  let fade;
  const pickTheme = (theme) => {
    const root = document.documentElement;
    root.classList.add("switching");
    clearTimeout(fade);
    fade = setTimeout(() => root.classList.remove("switching"), 500);
    apply(theme);
  };

  document.addEventListener("click", async (event) => {
    const shuffle = event.target.closest("[data-shuffle]");
    const pick = event.target.closest("[data-theme]");
    if (shuffle) {
      const all = await load();
      pickTheme(all[Math.floor(Math.random() * all.length)]);
    } else if (pick) {
      pickTheme("default" in pick.dataset ? null : await byName(pick.dataset.theme));
    }
  });

  // The theme picked on an earlier visit, already on the page from the
  // script in <head>; this names it and marks its button.
  let saved = null;
  try {
    saved = JSON.parse(localStorage.getItem(KEY));
  } catch {}
  apply(isTheme(saved) ? saved : null);
})();
