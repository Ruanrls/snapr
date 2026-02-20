import React, { createContext, use, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export type ScreenPositions =
  | "TopLeft"
  | "TopRight"
  | "BottomRight"
  | "BottomLeft"
  | "Center"
  | "Top"
  | "Right"
  | "Left"
  | "Bottom"
  | "Maximize";

export type KeyBinding = {
  modifiers: number;
  key: number;
};

export type Command = {
  key_binding: KeyBinding;
  position: ScreenPositions;
};

export type CommandContextType = {
  commands: Map<string, Command>;
  positionToCommand: Map<ScreenPositions, string>;
  insertCommand: (keyBinding: string, command: Command) => void;
  removeCommand: (keyBinding: string) => void;
};

type InvokeResponse = Promise<{
  commands: Record<string, Command>;
}>;

const commandContext = createContext<CommandContextType>(
  {} as CommandContextType,
);

export const CommandsProvider = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  const [positionToCommand, setPositionToCommand] = React.useState<
    Map<ScreenPositions, string>
  >(new Map());
  const [commands, setCommands] = React.useState<Map<string, Command>>(
    new Map(),
  );

  const insertCommand = (keyBinding: string, command: Command) => {
    setCommands((prev) => new Map(prev).set(keyBinding, command));
    setPositionToCommand((prev) => {
      const newMap = new Map(prev);
      newMap.set(command.position, keyBinding);
      return newMap;
    });

    invoke("save_config", {
      config: {
        commands: Object.fromEntries(
          new Map([...commands, [keyBinding, command]]).entries(),
        ),
      },
    });
  };

  const removeCommand = (keyBinding: string) => {
    const command = commands.get(keyBinding);
    if (!command) return;

    setCommands((prev) => {
      const newCommands = new Map(prev);
      newCommands.delete(keyBinding);
      return newCommands;
    });

    setPositionToCommand((prev) => {
      const newMap = new Map(prev);
      newMap.delete(command.position);
      return newMap;
    });
  };

  useEffect(() => {
    const fetchConfig = async () => {
      const { commands } = await invoke<InvokeResponse>("load_config");
      console.log("Loaded configuration:", commands);
      const commandsHash: Map<string, Command> = new Map();
      const positionMap: Map<ScreenPositions, string> = new Map();

      for (const [keyBinding, command] of Object.entries(commands)) {
        commandsHash.set(keyBinding, command);
        positionMap.set(command.position, keyBinding);
      }

      setCommands(commandsHash);
      setPositionToCommand(positionMap);
    };

    fetchConfig();
  }, []);

  return (
    <commandContext.Provider
      value={{ commands, positionToCommand, insertCommand, removeCommand }}
    >
      {children}
    </commandContext.Provider>
  );
};

export const useCommands = () => use(commandContext);
