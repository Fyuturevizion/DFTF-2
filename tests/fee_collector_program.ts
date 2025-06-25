import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FeeCollectorProgram } from "../target/types/fee_collector_program";

describe("fee_collector_program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace
    .FeeCollectorProgram as Program<FeeCollectorProgram>;

  it("accumulates fees", async () => {
    // TODO: add basic test
  });
});
