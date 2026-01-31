// -------- CONTROLS --------
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

// -------- POSTS --------
const posts = window.BLOG_POSTS;
const container = document.getElementById("posts-list");

posts.forEach(post => {
  const article = document.createElement("article");
  article.className = "group";

  article.innerHTML = `
    <header>
      <h2 class="text-2xl sm:text-3xl mb-2 text-[var(--text-primary)]">
        ${post.title}
      </h2>
      <div class="meta-text">
        ${post.created_at}
      </div>
    </header>

    <div class="text-[var(--text-primary)] leading-relaxed mb-4">
      <p>${post.content}</p>
    </div>

    <a href="https://advait.pro/blog/${post.filename}" class="read-more-link">
      Read full post <span class="read-more-arrow">&rarr;</span>
    </a>
  `;

  container.appendChild(article);
});

// -------- THEME / FONT --------
function setTheme(theme) {
  document.body.setAttribute("data-theme", theme);
  localStorage.setItem("blogTheme", theme);
}

function setFont(font) {
  document.body.classList.toggle("font-sans-system", font === "sans");
  document.body.classList.toggle("font-serif-times", font === "serif");
  localStorage.setItem("blogFont", font);
}

// -------- INIT --------
setTheme(localStorage.getItem("blogTheme") || "light");
setFont(localStorage.getItem("blogFont") || "sans");
