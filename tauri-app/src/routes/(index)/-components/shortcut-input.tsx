import { Kbd, KbdGroup } from "@/components/ui/kbd";
import { cn } from "@/lib/utils";
import { invoke } from "@tauri-apps/api/core";
import React, { type KeyboardEvent, useRef, useState } from "react";

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
		invoke("stop_listening_keyboard");
	};

	const handleBlur = () => {
		setModifiers(0);
		setActionKey(0);
		setIsRecording(false);
		invoke("start_listening_keyboard");
	};

	const keyCodeToDisplayString = (keyCode: number): string => {
		if (keyCode === 0) return "";

		const keyCodeMapper: Record<number, string> = {
			37: "←",
			38: "↑",
			39: "→",
			40: "↓",
			32: "Space",
			27: "Esc",
			8: "⌫",
			46: "Del",
			13: "↵",
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
			className={cn(
				"flex justify-center items-center h-8 min-w-36 px-3 rounded-lg cursor-pointer outline-none transition-all duration-200",
				isRecording
					? "bg-accent-blue/15 ring-2 ring-accent-blue/50"
					: modifiersParam === 0
						? "bg-muted/40 hover:bg-muted/60 ring-1 ring-border/50 hover:ring-border"
						: "hover:bg-muted/30",
			)}
		>
			{isRecording && modifiers === 0 && (
				<span className="text-xs text-muted-foreground">Type shortcut...</span>
			)}
			{!isRecording && modifiers === 0 && modifiersParam === 0 && (
				<span className="text-xs text-muted-foreground/60">
					Click to record
				</span>
			)}
			<KbdGroup className="justify-center items-center px-0 gap-0.5">
				{isRecording
					? renderKbdGroup(modifiers, keyCodeToDisplayString(actionKey))
					: renderKbdGroup(modifiersParam, keyCodeToDisplayString(actionKeyParam))}
			</KbdGroup>
		</div>
	);
};
