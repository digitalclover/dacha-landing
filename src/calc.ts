console.log('Calculator Time!');

type ShapeType = 'circle' | 'square';

const original = {
  shape: document.getElementById('originalShape') as HTMLSelectElement,
  value: document.getElementById('originalValue') as HTMLInputElement,
};

const target = {
  shape: document.getElementById('targetShape') as HTMLSelectElement,
  value: document.getElementById('targetValue') as HTMLInputElement,
};

const getLabel = (type: ShapeType) => (type === 'circle' ? 'Radius' : 'Width');

const updateLabels = () => {
  const ogShapeType = original.shape!.value as ShapeType;
  const ogLabel = getLabel(ogShapeType);
  original.shape.nextElementSibling!.textContent = ogLabel;
  const targetShapeType = target.shape?.value as ShapeType;
  const targetLabel = getLabel(targetShapeType);
  target.shape.nextElementSibling!.textContent = targetLabel;
};

const getArea = (source: 'original' | 'target', type: ShapeType) => {
  const isCircle = type === 'circle';
  const obj = source === 'original' ? original : target;
  const sourceValue = parseInt(obj.value!.value!);
  const value = isCircle ? sourceValue / 2 : sourceValue;
  const multiple = isCircle ? Math.PI : 1;
  return multiple * Math.pow(value, 2);
};

const calculate = () => {
  console.log('calculating...');
  const ogShapeType = original.shape!.value as ShapeType;
  console.log('ogShapeType', ogShapeType);
  const ogArea = getArea('original', ogShapeType);

  const targetShapeType = target.shape?.value as ShapeType;
  console.log('targetShapeType', targetShapeType);
  const targetValue = getArea('target', targetShapeType);

  const finalValue = (targetValue / ogArea).toLocaleString('ru', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
  document.getElementById('answer')!.textContent = finalValue;
};

const inputs = [
  ...Object.values({ ...original }),
  ...Object.values({ ...target }),
];

inputs.forEach((input) =>
  input?.addEventListener('change', () => {
    updateLabels();
    calculate();
  })
);
calculate();
