const slideConfigs = [
  { target: '75% 50%' },
  { target: '20% 40%' },
  { target: '65% 45%' },
  { target: '50% 50%' },
  { target: '50% 50%' },
  { target: '50% 50%' },
  { target: '50% 50%' },
];
const slideshowLength = slideConfigs.length;
let currentSlide = 1;

const getDefaultFontSize = () => {
  const element = document.createElement('div');
  element.style.width = '1rem';
  element.style.display = 'none';
  document.body.append(element);

  const widthMatch = window
    .getComputedStyle(element)
    .getPropertyValue('width')
    .match(/\d+/);

  element.remove();

  if (!widthMatch || widthMatch.length < 1) {
    return null;
  }

  const result = Number(widthMatch[0]);
  return !isNaN(result) ? result : null;
};

const defaultFontSize = getDefaultFontSize() || 16;

const getImage = (count: number) => {
  return `/images/slideshow/slide${count}_large.jpg`;
};

const getBackgroundRatio = (
  count: number
): Promise<'landscape' | 'portrait'> => {
  const image = new Image();
  return new Promise((res, rej) => {
    image.addEventListener('load', () => {
      const ratio =
        image.naturalWidth > image.naturalHeight ? 'landscape' : 'portrait';
      res(ratio);
    });
    image.addEventListener('error', (err) => rej(err));
    image.src = getImage(count);
  });
};

const getWindowRatio = () =>
  window.innerWidth > window.innerHeight ? 'landscape' : 'portrait';

const getNewSlide = async (count: number) => {
  const slide = document.createElement('div');
  const ratio = await getBackgroundRatio(count);
  slide.classList.add(`slide-${count}`);
  slide.classList.add('slide');
  slide.classList.add(ratio);
  slide.style.backgroundImage = `url('${getImage(count)}')`;

  slide.style.zIndex = `${count}`;
  const isOdd = count % 2 === 1;
  slide.style.backgroundPosition = isOdd ? '40% 30%' : '60% 70%';

  return slide;
};

const handlePrevSlide = (container: HTMLElement, count: number) => {
  const prevCount = count === 1 ? slideshowLength : count - 1;
  const prevSlide: HTMLElement | null = container.querySelector(
    `.slide-${prevCount}`
  );
  if (prevSlide) {
    prevSlide!.style.zIndex = `${count + 1}`;
    prevSlide.classList.add('fade-out');
    setTimeout(() => {
      container.removeChild(prevSlide);
    }, 1_000);
  }
};

const slideshowInterval = () =>
  setInterval(async () => {
    const lastSlide = currentSlide === slideshowLength;
    currentSlide = lastSlide ? 1 : currentSlide + 1;
    await animateBackground(currentSlide);
  }, 3_000);

const animateBackground = async (count = 1) => {
  const newSlide = await getNewSlide(count);
  const container = document.querySelector('header');
  container!.append(newSlide);
  const center = `50% 50%`;
  checkRatios();

  setTimeout(() => {
    newSlide.style.backgroundPosition = `${
      slideConfigs[count - 1].target || center
    }`;
  }, 100);
  handlePrevSlide(container!, count);
};

const startSlideshow = async () => {
  await animateBackground();
  slideshowInterval();
};

const checkLazyBgImages = (entry: IntersectionObserverEntry) => {
  const isOverlapping = entry.intersectionRatio > 0;
  const isLazy = entry.target.classList.contains('lazy');
  const isLoaded = entry.target.classList.contains('w_image');
  if (isOverlapping && isLazy && !isLoaded) {
    entry.target.classList.add('w-image');
  }
};

const intersectionObserver = new IntersectionObserver(
  (entries) => {
    entries.forEach(checkLazyBgImages);
  },
  {
    root: null,
    rootMargin: '10%',
    threshold: 0,
  }
);

const checkRatios = () => {
  const slides = document.querySelectorAll('.slide');
  slides.forEach((slide) => {
    const imageRatio = slide.classList.contains('landscape')
      ? 'landscape'
      : 'portrait';
    const windowRatio = getWindowRatio();
    if (imageRatio === windowRatio) {
      slide.classList.remove(imageRatio);
      const newRatio = imageRatio === 'landscape' ? 'portrait' : 'landscape';
      slide.classList.add(newRatio);
    }
  });
};

const movePageScroll = (start: number) => (timestamp: number) => {
  if (!start) start = timestamp;
  const elapsedTime = timestamp - start;
  const percent = elapsedTime / 1_000;
  const distance = Math.floor((window.scrollY * percent));
  const newPosition = window.scrollY - distance;
  if (newPosition > 0) {
    window.scrollTo({ top: newPosition });
    window.requestAnimationFrame(movePageScroll(start));
  } else {
    window.scrollTo({ top: 0 });
  }
};

const handleHoverButtons = () => {
  const topButton = document.querySelector('#backToTop');
  const langButton = document.querySelector('#lang a');
  topButton?.addEventListener('click', () =>
    window.requestAnimationFrame(movePageScroll(0))
  );
  window.onscroll = () => {
    const enable = window.scrollY > window.innerHeight * 2;
    const visible = topButton?.classList.contains('show');
    if (enable && !visible) {
      topButton?.classList.add('show');
      langButton?.classList.remove('show');
    }
    if (!enable && visible) {
      topButton?.classList.remove('show');
      langButton?.classList.add('show');
    }
  };
};

window.onload = async () => {
  // await startSlideshow();
  // checkRatios();
  const lazyImages = document.querySelectorAll('.lazy');
  lazyImages.forEach((image) => intersectionObserver.observe(image));
  handleHoverButtons();
};

// window.addEventListener('resize', checkRatios);
