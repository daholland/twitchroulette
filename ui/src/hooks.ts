import { useEffect, useState } from 'preact/hooks';

export function useKeyPress(targetKey: string): boolean {
  const [keyPressed, setKeyPressed] = useState(false);

  function downHandler(e: any): void {
    if (e.key === targetKey) {
      setKeyPressed(true);
    }
  }

  const upHandler = (e: any): void => {
    if (e.key === targetKey) {
      setKeyPressed(false);
    }
  };

  useEffect(() => {
    window.addEventListener('keydown', downHandler);
    window.addEventListener('keyup', upHandler);

    return () => {
      window.removeEventListener('keydown', downHandler);
      window.removeEventListener('keyup', upHandler);
    };
  }, []);

  return keyPressed;
}
