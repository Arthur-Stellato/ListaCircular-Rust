// Intersection Observer para os efeitos de fade-in nas seções
const obs = new IntersectionObserver((entries) => {
  entries.forEach(e => { 
    if (e.isIntersecting) e.target.classList.add('visible'); 
  });
}, { threshold: 0.08 });

document.querySelectorAll('.fade-in').forEach(el => obs.observe(el));

// Monitoramento do scroll para ativar os links corretos na Navigation Bar
const sections = document.querySelectorAll('section[id]');
const navLinks = document.querySelectorAll('nav a');

window.addEventListener('scroll', () => {
  let cur = '';
  sections.forEach(s => { 
    if (window.scrollY >= s.offsetTop - 80) cur = s.id; 
  });
  
  navLinks.forEach(a => {
    a.style.color = a.getAttribute('href') === '#' + cur ? 'var(--white)' : '';
    a.style.borderBottomColor = a.getAttribute('href') === '#' + cur ? 'var(--accent)' : 'transparent';
  });
});