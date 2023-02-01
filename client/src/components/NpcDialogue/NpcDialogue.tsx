import { useState } from "react";
import {
  DialogueContent,
  DialogueMap,
  Position,
  ServerMessageSingleClient,
} from "../../utility/types";
import "./NpcDialogue.css";

export interface NpcDialogueProps {
  nameAndDialogueMap: Extract<
    ServerMessageSingleClient,
    { type: "showDialogue" }
  >["content"];
  onClose: () => void;
}

export const NpcDialogue: React.FC<NpcDialogueProps> = ({
  nameAndDialogueMap,
  onClose,
}) => {
  const { entityName, dialogueMap } = nameAndDialogueMap;
  if (!dialogueMap[0]) {
    console.error("You forgot to start with a 0 index in your dialogue map");
    console.error(dialogueMap);
    throw Error;
  }
  const [currentDialogue, setCurrentDialogue] = useState<DialogueContent>(
    dialogueMap[0]
  );

  const {
    text,
    response_1_id,
    response_1_text,
    response_2_id,
    response_2_text,
  } = currentDialogue;

  return (
    <div className="dialogue-container">
      <div className="sprite-and-name">
        {/* <img className="dialogue-portrait" src="sprites/v2/objectRedSoda.png" /> */}
        <p>{entityName}</p>
      </div>
      <p className="dialogue-text">{text}</p>

      <div className="button-container">
        {response_1_text && (
          <button
            onClick={() => {
              if (response_1_id !== undefined) {
                const nextDialogueContent = dialogueMap[response_1_id];
                if (!nextDialogueContent) {
                  console.error(
                    "Missing next connection in dialogue map: ",
                    response_1_id
                  );
                  console.error(dialogueMap);
                  throw Error;
                }
                setCurrentDialogue(nextDialogueContent);
              }
            }}
          >
            {response_1_text}
          </button>
        )}

        {response_2_text && (
          <button
            onClick={() => {
              if (response_2_id !== undefined) {
                const nextDialogueContent = dialogueMap[response_2_id];
                if (!nextDialogueContent) {
                  console.error(
                    "Missing next connection in dialogue map: ",
                    response_2_id
                  );
                  console.error(dialogueMap);
                  throw Error;
                }
                setCurrentDialogue(nextDialogueContent);
              }
            }}
          >
            {response_2_text}
          </button>
        )}

        {!response_1_text && !response_1_text && (
          <button
            onClick={() => {
              onClose();
            }}
          >
            Done
          </button>
        )}
      </div>
    </div>
  );
};
