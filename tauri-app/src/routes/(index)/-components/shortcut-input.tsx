import { Kbd, KbdGroup } from "@/components/ui/kbd";
import React, { KeyboardEvent, useRef, useState } from "react";

enum Modifiers {
  Control = 12,
  Shift = 13,
  Meta = 14,
  Alt = 15,
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

const eventKeyToDisplayString = (key: string) => {
  const mapper: Record<string, string> = {
    ArrowUp: "↑",
    ArrowDown: "↓",
    ArrowLeft: "←",
    ArrowRight: "→",
    " ": "Space",
    Escape: "Esc",
    Backspace: "⌫",
    Delete: "Del",
    Enter: "↵",
  };

  return mapper[key] || key;
};

export type InputData = {
  modifiers: number[];
  actionKey: string;
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

  const [modifiers, setModifiers] = useState<Set<number>>(
    new Set(modifiersParam)
  );
  const [actionKey, setActionKey] = useState(actionKeyParam);

  const handleKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    event.stopPropagation();
    event.preventDefault();

    if (!isRecording) {
      return;
    }

    const modifierCode = eventKeyToModifierCode(event.code);
    if (modifierCode) {
      setModifiers((current) => {
        current.add(modifierCode);
        return new Set(current);
      });
    } else {
      if (modifiers.size === 0) {
        setActionKey("");
        return;
      }

      const key = eventKeyToDisplayString(event.key);
      setActionKey(key);
      onRecord({
        modifiers: Array.from(modifiers.values()),
        actionKey: key,
      });
      divRef.current?.blur();
    }
  };

  const handleKeyUp = (event: KeyboardEvent<HTMLDivElement>) => {
    const modifierCode = eventKeyToModifierCode(event.code);
    if (modifierCode) {
      setModifiers((current) => {
        current.delete(modifierCode);
        return new Set(current);
      });
    }
  };

  const handleClick = () => {
    divRef.current?.focus();
    setModifiers(new Set());
    setActionKey("");
    setIsRecording(true);
  };

  const handleBlur = () => {
    setModifiers(new Set());
    setActionKey("");
    setIsRecording(false);
  };

  const renderKbdGroup = (modifiersParam: number[], actionKeyParam: string) => {
    const kbdKeys = modifiersParam.sort().map((element, index) => (
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
        </React.Fragment>
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
      {isRecording && !modifiers.size && (
        <span className="text-sm text-foreground">type to record...</span>
      )}
      {!isRecording && !modifiers.size && !modifiersParam.length && (
        <span className="text-sm text-foreground">press to record</span>
      )}
      <KbdGroup className="justify-center items-center px-0 gap-0.5">
        {isRecording
          ? renderKbdGroup(Array.from(modifiers.values()), actionKey)
          : renderKbdGroup(modifiersParam, actionKeyParam)}
      </KbdGroup>
    </div>
  );
};
