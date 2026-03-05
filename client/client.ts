async function runToDoList() {
  console.log("🚀 Iniciando Lista de Tareas...");

  // 1. Derivar la dirección de la PDA
  const [listaPDA] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("lista_tareas"), pg.wallet.publicKey.toBuffer()],
    pg.program.programId
  );

  try {
    // 2. CREAR LISTA
    const cuentaExistente = await pg.connection.getAccountInfo(listaPDA);
    if (!cuentaExistente) {
      console.log("📝 Creando nueva lista...");
      await pg.program.methods
        .crearLista("Tareas de Johnny")
        .accounts({
          owner: pg.wallet.publicKey,
          lista: listaPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
    }

    // 3. AGREGAR TAREA
    console.log("➕ Agregando tarea: Estudiar Solana...");
    await pg.program.methods
      .agregarTarea("Estudiar Solana", 5) // descripcion, prioridad
      .accounts({
        owner: pg.wallet.publicKey,
        lista: listaPDA,
      })
      .rpc();

    // 4. VER TAREAS
    const datos = await pg.program.account.listaTareas.fetch(listaPDA);
    console.log("\n--- TAREAS ACTUALES ---");
    datos.tareas.forEach((t, i) => {
      console.log(`${i + 1}. ${t.descripcion} | Prioridad: ${t.prioridad} | Hecho: ${t.completada}`);
    });

  } catch (error) {
    console.error("❌ Error:", error);
  }
}

runToDoList();
