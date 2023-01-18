console.log((2n ** 1000n).toString().split('').map($ => +$).reduce((acc, next) => acc + next));
