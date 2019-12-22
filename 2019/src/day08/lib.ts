export const fewestZeros = (acc, next) => {
  const zeroes_acc = acc.filter(e => e === 0).length;
  const zeroes_next = next.filter(e => e === 0).length;
  if (zeroes_acc < zeroes_next) {
    return acc;
  } else {
    return next;
  }
};

export const numLayers = (input, layerSize) => {
  if (input.length % layerSize !== 0) {
    throw "Corrupt input";
  } else {
    return input.length / layerSize;
  }
};
