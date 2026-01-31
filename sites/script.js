
// ---------------- CONTROLS INJECTION ----------------

document.body.insertAdjacentHTML("afterbegin", `
  <div class="controls-wrapper">
    <div class="control-group" id="theme-toggles">
      <button class="theme-btn theme-btn-light" onclick="setTheme('light')"></button>
      <button class="theme-btn theme-btn-sepia" onclick="setTheme('sepia')"></button>
      <button class="theme-btn theme-btn-dark" onclick="setTheme('dark')"></button>
    </div>

    <div class="control-group" id="font-toggles">
      <button class="font-btn font-sans-system" onclick="setFont('sans')">Aa</button>
      <button class="font-btn font-serif-times" onclick="setFont('serif')">Aa</button>
    </div>
  </div>
`);

// ---------------- RENDER POST ----------------

const post = window.BLOG_POST;
const container = document.getElementById("posts-container");

container.innerHTML = `
  <article>
    <h1 class="text-3xl sm:text-4xl mb-3">${post.title}</h1>
    <span class="text-sm text-[var(--text-secondary)]">${post.created_at}</span>
    <p class="mt-6">${post.content}</p>
  </article>
`;

// ---------------- THEME / FONT LOGIC ----------------

function setTheme(theme) {
  document.body.setAttribute("data-theme", theme);
  localStorage.setItem("blogTheme", theme);
  updateThemeButtons(theme);
}

function setFont(font) {
  document.body.classList.toggle("font-sans-system", font === "sans");
  document.body.classList.toggle("font-serif-times", font === "serif");
  localStorage.setItem("blogFont", font);
}

function updateThemeButtons(theme) {
  document.querySelectorAll(".theme-btn").forEach(btn =>
    btn.classList.toggle("active", btn.classList.contains(`theme-btn-${theme}`))
  );
}

// ---------------- INIT ----------------

const savedTheme = localStorage.getItem("blogTheme") || "light";
const savedFont = localStorage.getItem("blogFont") || "sans";

setTheme(savedTheme);
setFont(savedFont);
