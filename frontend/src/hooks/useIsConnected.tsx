import { useEffect, useState } from "react";
import { useFuel } from "./useFuel"; // Импортируем useFuel


export function useIsConnected() {
  const [fuel] = useFuel(); // Используем хук useFuel для получения fuel

  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    async function handleConnection() {
      if (fuel) {
        // Используем fuel из хука useFuel
        const isConnected = await fuel.isConnected();
        setIsConnected(isConnected);
      }
    }

    if (fuel) {
      // Используем fuel из хука useFuel
      handleConnection();
    }

    fuel?.on(fuel.events.connection, handleConnection); // Используем fuel из хука useFuel
    return () => {
      fuel?.off(fuel.events.connection, handleConnection); // Используем fuel из хука useFuel
    };
  }, [fuel]);

  return [isConnected];
}
