const smokeContainer = document.querySelector('.smoke-background');

const animations = [
  { name: 'smokeFade1', duration: '14s' },
  { name: 'smokeFade2', duration: '12s' },
  { name: 'smokeFade3', duration: '16s' }
];

let animationIndex = 0;

function createSmoke() {
  const smoke = document.createElement('img');
  smoke.src = './assets/smoke.png';
  smoke.className = 'smoke';

  // Random position across the screen
  smoke.style.left = `${Math.random() * 100}%`;
  smoke.style.bottom = `${Math.random() * 40}%`;

  // Assign animation properties
  const animation = animations[animationIndex];
  smoke.style.animationName = animation.name;
  smoke.style.animationDuration = animation.duration;
  smoke.style.animationIterationCount = '1'; // Run once
  smoke.style.animationTimingFunction = 'ease-in-out';

  // Move to the next animation for the next smoke
  animationIndex = (animationIndex + 1) % animations.length;

const delay = 1000 + Math.random() * 1000;
  setTimeout(() => {
    smokeContainer.appendChild(smoke); // Add to DOM
    smoke.addEventListener('animationend', () => smoke.remove()); // Attach listener
  }, delay);

}

// Spawn smoke every 700ms for a gentle, sparse effect
setInterval(createSmoke, 500);

// Initial burst of smoke
for (let i = 0; i < 7; i++) {
  createSmoke();
}