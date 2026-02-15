# {% if config.title %}{{ config.title }}{% else %}Hi there, I'm {{ user.name | default(value=user.login) }} 👋{% endif %}

{% if config.subtitle %}{{ config.subtitle }}{% endif %}

{% if user.bio %}
> {{ user.bio }}
{% endif %}

## 📊 GitHub Stats

{% if config.show_stats %}
- 🌟 **{{ total_stars }}** total stars across all repositories
- 📦 **{{ user.public_repos }}** public repositories
- 👥 **{{ user.followers }}** followers
- 👤 Following **{{ user.following }}** users
{% endif %}

{% if config.show_languages and languages | length > 0 %}
## 💻 Languages & Technologies

{% for lang in languages %}
- {{ lang }}
{% endfor %}
{% endif %}

{% if config.show_recent_repos and repos | length > 0 %}
## 🚀 Recent Projects

{% for repo in repos | slice(end=5) %}
### [{{ repo.name }}]({{ repo.html_url }})
{% if repo.description %}{{ repo.description }}{% endif %}
- ⭐ Stars: {{ repo.stargazers_count }}
- 🍴 Forks: {{ repo.forks_count }}
{% if repo.language %}- 💻 Language: {{ repo.language }}{% endif %}

{% endfor %}
{% endif %}

{% for section in config.custom_sections %}
## {{ section.title }}

{{ section.content }}

{% endfor %}

## 📫 How to reach me

{% if user.email %}- 📧 Email: {{ user.email }}{% endif %}
{% if user.blog %}- 🌐 Website: {{ user.blog }}{% endif %}
{% if user.twitter_username %}- 🐦 Twitter: [@{{ user.twitter_username }}](https://twitter.com/{{ user.twitter_username }}){% endif %}
{% if user.location %}- 📍 Location: {{ user.location }}{% endif %}

---

<p align="center">
  <img src="https://komarev.com/ghpvc/?username={{ user.login }}&color=blueviolet&style=flat-square&label=Profile+Views" alt="Profile views" />
</p>

*This README was generated using [github-profile-rust](https://github.com/{{ user.login }}/github-profile-rust)*
