import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RouterProgram } from "../target/types/router_program";

describe("router_program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.RouterProgram as Program<RouterProgram>;

  it("initialized", async () => {
    // TODO: add basic test
  });
});
