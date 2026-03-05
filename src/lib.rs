use anchor_lang::prelude::*;

// Este ID se actualizará solo cuando hagas clic en "Build"
declare_id!("GBJUHS8DZyucsmNbS5DhQmC9hbVE3tqrrJnerKA48uz3");

#[program]
pub mod lista_tareas {
    use super::*;

// 1. Corrige la función crear_lista
pub fn crear_lista(ctx: Context<NuevaLista>, nombre_lista: String) -> Result<()> {
    let owner = ctx.accounts.owner.key();
    let tareas: Vec<Tarea> = Vec::new();

    ctx.accounts.lista.set_inner(ListaTareas {
        owner,
        nombre: nombre_lista.clone(), // AGREGAMOS .clone() AQUÍ
        tareas,
    });

    msg!("¡Lista de tareas '{}' creada con éxito!", nombre_lista); // Ahora sí funcionará
    Ok(())
}

// 2. Corrige la función agregar_tarea
pub fn agregar_tarea(ctx: Context<GestionTareas>, descripcion: String, prioridad: u8) -> Result<()> {
    let tarea = Tarea {
        descripcion: descripcion.clone(), // AGREGAMOS .clone() AQUÍ
        prioridad,
        completada: false,
    };

    ctx.accounts.lista.tareas.push(tarea);
    msg!("Tarea agregada: {}", descripcion); // Ahora sí funcionará
    Ok(())
}

    // READ: Ver tareas en logs
    pub fn ver_tareas(ctx: Context<GestionTareas>) -> Result<()> {
        msg!("Lista: {}", ctx.accounts.lista.nombre);
        for tarea in &ctx.accounts.lista.tareas {
            let estado = if tarea.completada { "Completada" } else { "Pendiente" };
            msg!("Tarea: {} | Prioridad: {} | Estado: {}", tarea.descripcion, tarea.prioridad, estado);
        }
        Ok(())
    }

    // UPDATE: Marcar tarea como completada o cambiar prioridad
    pub fn actualizar_tarea(ctx: Context<GestionTareas>, descripcion: String, nueva_prioridad: u8, completada: bool) -> Result<()> {
        let tareas = &mut ctx.accounts.lista.tareas;
        
        for t in tareas.iter_mut() {
            if t.descripcion == descripcion {
                t.prioridad = nueva_prioridad;
                t.completada = completada;
                msg!("Tarea '{}' actualizada.", descripcion);
                return Ok(());
            }
        }
        Err(Errores::TareaNoEncontrada.into())
    }

    // DELETE: Eliminar una tarea
    pub fn eliminar_tarea(ctx: Context<GestionTareas>, descripcion: String) -> Result<()> {
        let tareas = &mut ctx.accounts.lista.tareas;
        let index = tareas.iter().position(|t| t.descripcion == descripcion);

        if let Some(i) = index {
            tareas.remove(i);
            msg!("Tarea '{}' eliminada.", descripcion);
            Ok(())
        } else {
            Err(Errores::TareaNoEncontrada.into())
        }
    }
}

#[error_code]
pub enum Errores {
    #[msg("La tarea solicitada no existe en la lista.")]
    TareaNoEncontrada,
}

#[account]
#[derive(InitSpace)]
pub struct ListaTareas {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre: String,
    #[max_len(15)] // Capacidad para 15 tareas
    pub tareas: Vec<Tarea>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Tarea {
    #[max_len(50)]
    pub descripcion: String,
    pub prioridad: u8,
    pub completada: bool,
}

#[derive(Accounts)]
pub struct NuevaLista<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = ListaTareas::INIT_SPACE + 8,
        seeds = [b"lista_tareas", owner.key().as_ref()],
        bump
    )]
    pub lista: Account<'info, ListaTareas>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionTareas<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"lista_tareas", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub lista: Account<'info, ListaTareas>,
}
