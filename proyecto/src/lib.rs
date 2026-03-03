use anchor_lang::prelude::*;

declare_id!("AeJXgVqtWz1n1vw5XiTaEnf7mvJ69jjtLuU2txeshsbu");

#[program]
pub mod tienda_cosmeticos {
    use super::*;

    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String, ubicacion: String, hora_apertura: u8, hora_cierre: u8,) -> Result<()> {
        require!(hora_apertura < 24 && hora_cierre < 24, Errores::HoraInvalida);
        context.accounts.tienda.set_inner(Tienda {
            owner: context.accounts.owner.key(),
            nombre,
            ubicacion,
            hora_apertura,
            hora_cierre,
            productos: Vec::new(),
            total_ventas: 0,
        });
        Ok(())
    }

    pub fn agregar_producto(context: Context<ModificarTienda>, nombre: String, categoria: String, precio: u64, stock: u16,) -> Result<()> {
        let tienda = &mut context.accounts.tienda;
        for producto in &tienda.productos {
            require!(producto.nombre != nombre, Errores::ProductoYaExiste);
        }
        let nuevo_producto = Producto {
            nombre,
            categoria,
            precio,
            stock,
            disponible: true,
        };
        tienda.productos.push(nuevo_producto);
        Ok(())
    }

        pub fn comprar_producto( context: Context<ModificarTienda>, nombre: String,) -> Result<()> {
    let tienda = &mut context.accounts.tienda;

    for i in 0..tienda.productos.len() {
        if tienda.productos[i].nombre == nombre {
            require!(tienda.productos[i].stock > 0, Errores::SinStock);
            let precio = tienda.productos[i].precio;
            tienda.productos[i].stock -= 1;
            tienda.total_ventas += precio;
            if tienda.productos[i].stock == 0 {
                tienda.productos[i].disponible = false;
            }
            return Ok(());
        }
    }
    Err(Errores::ProductoNoExiste.into())
}
    pub fn actualizar_precio(context: Context<ModificarTienda>,nombre: String,nuevo_precio: u64,) -> Result<()> {
        let tienda = &mut context.accounts.tienda;
        for producto in &mut tienda.productos {
            if producto.nombre == nombre {
                producto.precio = nuevo_precio;
                return Ok(());
            }
        }
        Err(Errores::ProductoNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("No eres el dueño de la tienda")]
    NoEresElOwner,

    #[msg("El producto no existe")]
    ProductoNoExiste,

    #[msg("El producto ya existe")]
    ProductoYaExiste,

    #[msg("No hay stock disponible")]
    SinStock,

    #[msg("Hora inválida (0-23)")]
    HoraInvalida,
}

#[account]
#[derive(InitSpace)]
pub struct Tienda {

    pub owner: Pubkey,

    #[max_len(60)]
    pub nombre: String,

    #[max_len(100)]
    pub ubicacion: String,

    pub hora_apertura: u8,
    pub hora_cierre: u8,

    #[max_len(40)]
    pub productos: Vec<Producto>,

    pub total_ventas: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, Debug, PartialEq)]
pub struct Producto {

    #[max_len(60)]
    pub nombre: String,

    #[max_len(40)]
    pub categoria: String,

    pub precio: u64,

    pub stock: u16,

    pub disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaTienda<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Tienda::INIT_SPACE + 8,
        seeds = [b"tienda_cosmeticos", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarTienda<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"tienda_cosmeticos", owner.key().as_ref()],
        bump,
        constraint = tienda.owner == owner.key() @ Errores::NoEresElOwner
    )]
    pub tienda: Account<'info, Tienda>,
}
