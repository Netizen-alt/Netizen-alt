<div align="center">

# {% if config.title %}{{ config.title }}{% else %}👋 Hi, I'm {{ user.name | default(value=user.login) }}{% endif %}

{% if config.subtitle %}
### {{ config.subtitle }}
{% endif %}

{% if user.bio %}
<p><em>{{ user.bio }}</em></p>
{% endif %}

<p>
  <a href="{{ user.html_url }}">
    <img src="https://img.shields.io/github/followers/{{ user.login }}?label=Followers&style=for-the-badge&logo=github&color=blue" alt="GitHub Followers" />
  </a>
  <img src="https://komarev.com/ghpvc/?username={{ user.login }}&label=Profile%20Views&color=blueviolet&style=for-the-badge" alt="Profile Views" />
  <img src="https://img.shields.io/badge/Total%20Stars-{{ total_stars }}-yellow?style=for-the-badge&logo=github" alt="Total Stars" />
</p>

</div>

---

{% if config.show_stats %}
## 📊 GitHub Statistics

<div align="center">
  <img height="180em" src="https://github-readme-stats.vercel.app/api?username={{ user.login }}&show_icons=true&theme=tokyonight&include_all_commits=true&count_private=true&hide_border=true&bg_color=0d1117"/>
  <img height="180em" src="https://github-readme-stats.vercel.app/api/top-langs/?username={{ user.login }}&layout=compact&langs_count=8&theme=tokyonight&hide_border=true&bg_color=0d1117"/>
</div>

<br/>

<div align="center">
  <img src="https://github-readme-streak-stats.herokuapp.com/?user={{ user.login }}&theme=tokyonight&hide_border=true&background=0d1117" alt="GitHub Streak" />
</div>

<br/>

### 🏆 Quick Stats

<div align="center">

| 📦 Public Repos | ⭐ Total Stars | 👥 Followers | 👤 Following |
|:---------------:|:--------------:|:------------:|:------------:|
| **{{ user.public_repos }}** | **{{ total_stars }}** | **{{ user.followers }}** | **{{ user.following }}** |

</div>
{% endif %}

---

{% if config.show_languages and languages | length > 0 %}
## 💻 Tech Stack & Languages

<div align="center">

{% for lang in languages %}
![{{ lang }}](https://img.shields.io/badge/-{{ lang }}-05122A?style=for-the-badge&logo={{ lang | lower }}){% if not loop.last %} {% endif %}
{% endfor %}

</div>
{% endif %}

---

{% if config.show_recent_repos and repos | length > 0 %}
## 🚀 Recent Projects

### 📌 Recent Activity

{% for repo in repos | slice(end=5) %}
- 🔨 **[{{ repo.name }}]({{ repo.html_url }})** {% if repo.language %}- `{{ repo.language }}`{% endif %}
  {% if repo.description %}<br/>  {{ repo.description }}{% endif %}
  <br/>  ⭐ {{ repo.stargazers_count }} stars | 🍴 {{ repo.forks_count }} forks
{% endfor %}
{% endif %}

---

{% for section in config.custom_sections %}
## {{ section.title }}

<div align="center">

{{ section.content }}

</div>

{% endfor %}

{% if config.custom_sections | length > 0 %}
---
{% endif %}

## 📫 Connect With Me

<div align="center">

{% if user.blog %}
[![Website](https://img.shields.io/badge/Website-{{ user.blog | replace(from="https://", to="") | replace(from="http://", to="") }}-blue?style=for-the-badge&logo=google-chrome)]({{ user.blog }})
{% endif %}
{% if user.twitter_username %}
[![Twitter](https://img.shields.io/badge/Twitter-@{{ user.twitter_username }}-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white)](https://twitter.com/{{ user.twitter_username }})
{% endif %}
{% if user.email %}
[![Email](https://img.shields.io/badge/Email-{{ user.email }}-D14836?style=for-the-badge&logo=gmail&logoColor=white)](mailto:{{ user.email }})
{% endif %}
[![GitHub](https://img.shields.io/badge/GitHub-{{ user.login }}-181717?style=for-the-badge&logo=github)]({{ user.html_url }})
{% if user.location %}

📍 **Location:** {{ user.location }}
{% endif %}

</div>

---

<div align="center">

### 🎯 Profile Activity Graph

<img src="https://github-readme-activity-graph.vercel.app/graph?username={{ user.login }}&theme=tokyo-night&hide_border=true&bg_color=0d1117" alt="Activity Graph" />

<br/><br/>

### 🏆 GitHub Trophies

<img src="https://github-profile-trophy.vercel.app/?username={{ user.login }}&theme=tokyonight&no-frame=true&no-bg=true&margin-w=4&margin-h=4" alt="GitHub Trophies" />

</div>

---

<div align="center">
  <img src="https://capsule-render.vercel.app/api?type=waving&color=gradient&height=100&section=footer" alt="Footer Wave" />
  
  <br/>
  
  <sub>💖 Generated with [github-profile-rust](https://github.com/{{ user.login }}/github-profile-rust)</sub>
</div>
