import { Kbd, KbdGroup } from "@/components/ui/kbd";
import React, { KeyboardEvent, useRef, useState } from "react";

enum Modifiers {
  Control = 1,
  Shift = 1 << 1,
  Meta = 1 << 2,
  Alt = 1 << 3,
}

const eventKeyToModifierCode = (key: string) => {
  const mapper: Record<string, Modifiers> = {
    ControlLeft: Modifiers.Control,
    ControlRight: Modifiers.Control,
    ShiftLeft: Modifiers.Shift,
    ShiftRight: Modifiers.Shift,
    MetaLeft: Modifiers.Meta,
    MetaRight: Modifiers.Meta,
    AltLeft: Modifiers.Alt,
    AltRight: Modifiers.Alt,
  };

  return mapper[key] || undefined;
};

const modifierToChar = (code: Modifiers) => {
  const mapper: Record<Modifiers, string> = {
    [Modifiers.Control]: "Ctrl",
    [Modifiers.Shift]: "Shift",
    [Modifiers.Meta]: "Meta",
    [Modifiers.Alt]: "Alt",
  };

  return mapper[code] || "";
};

export type InputData = {
  modifiers: number;
  actionKey: number;
};

type ShortcutInputProps = InputData & {
  onRecord: (data: InputData) => void;
};
export const ShortcutInput = ({
  modifiers: modifiersParam,
  actionKey: actionKeyParam,
  onRecord,
}: ShortcutInputProps) => {
  const divRef = useRef<HTMLDivElement>(null);

  const [isRecording, setIsRecording] = useState(false);

  const [modifiers, setModifiers] = useState<number>(modifiersParam);
  const [actionKey, setActionKey] = useState<number>(actionKeyParam);

  const handleKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    event.stopPropagation();
    event.preventDefault();

    if (!isRecording) {
      return;
    }

    const modifierCode = eventKeyToModifierCode(event.code);
    if (modifierCode) {
      setModifiers((current) => current | modifierCode);
    } else {
      if (modifiers === 0) {
        setActionKey(0);
        return;
      }

      setActionKey(event.keyCode);
      onRecord({
        modifiers,
        actionKey: event.keyCode,
      });
      divRef.current?.blur();
    }
  };

  const handleKeyUp = (event: KeyboardEvent<HTMLDivElement>) => {
    const modifierCode = eventKeyToModifierCode(event.code);
    if (modifierCode) {
      setModifiers((current) => current & ~modifierCode);
    }
  };

  const handleClick = () => {
    divRef.current?.focus();
    setModifiers(0);
    setActionKey(0);
    setIsRecording(true);
  };

  const handleBlur = () => {
    setModifiers(0);
    setActionKey(0);
    setIsRecording(false);
  };

  const keyCodeToDisplayString = (keyCode: number): string => {
    if (keyCode === 0) return "";

    // Map key codes to display strings for special keys
    const keyCodeMapper: Record<number, string> = {
      37: "←", // ArrowLeft
      38: "↑", // ArrowUp
      39: "→", // ArrowRight
      40: "↓", // ArrowDown
      32: "Space",
      27: "Esc",
      8: "⌫", // Backspace
      46: "Del",
      13: "↵", // Enter
    };

    return keyCodeMapper[keyCode] || String.fromCharCode(keyCode);
  };

  const modifierBitmaskToArray = (bitmask: number): Modifiers[] => {
    const result: Modifiers[] = [];
    if (bitmask & Modifiers.Control) result.push(Modifiers.Control);
    if (bitmask & Modifiers.Shift) result.push(Modifiers.Shift);
    if (bitmask & Modifiers.Meta) result.push(Modifiers.Meta);
    if (bitmask & Modifiers.Alt) result.push(Modifiers.Alt);
    return result;
  };

  const renderKbdGroup = (modifiersBitmask: number, actionKeyParam: string) => {
    const modifiersArray = modifierBitmaskToArray(modifiersBitmask);
    const kbdKeys = modifiersArray.map((element, index) => (
      <React.Fragment key={element}>
        <span>{index > 0 && "+"}</span>
        <Kbd>{modifierToChar(element)}</Kbd>
      </React.Fragment>
    ));

    if (actionKeyParam) {
      kbdKeys.push(
        <React.Fragment key={actionKeyParam}>
          <span>+</span>
          <Kbd>{actionKeyParam}</Kbd>
        </React.Fragment>,
      );
    }

    return kbdKeys;
  };

  return (
    <div
      ref={divRef}
      tabIndex={0}
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      onKeyUp={handleKeyUp}
      onBlur={handleBlur}
      className="flex justify-center items-center w-52 h-7 border border-accent rounded-sm focus:ring-2 focus:ring-ring focus:ring-offset-2 cursor-pointer"
    >
      {isRecording && modifiers === 0 && (
        <span className="text-sm text-foreground">type to record...</span>
      )}
      {!isRecording && modifiers === 0 && modifiersParam === 0 && (
        <span className="text-sm text-foreground">press to record</span>
      )}
      <KbdGroup className="justify-center items-center px-0 gap-0.5">
        {isRecording
          ? renderKbdGroup(modifiers, keyCodeToDisplayString(actionKey))
          : renderKbdGroup(modifiersParam, keyCodeToDisplayString(actionKeyParam))}
      </KbdGroup>
    </div>
  );
};
