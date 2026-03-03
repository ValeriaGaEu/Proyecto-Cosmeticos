[💄 BeautyChain — Gestión de Tienda de Cosméticos en Solana

BeautyChain es un programa on-chain desarrollado en Rust con el framework Anchor sobre la blockchain de Solana.

Permite a dueños de tiendas de cosméticos gestionar su negocio de forma descentralizada, segura e inmutable.

📌 ¿Qué hace el proyecto?

BeautyChain implementa un sistema de gestión para una tienda de cosméticos que permite:

Crear una tienda vinculada a tu wallet (propietario)

Registrar productos (labiales, bases, perfumes, etc.)

Actualizar precios

Modificar stock

Eliminar productos

Consultar información almacenada en la blockchain

Cada tienda y cada producto son cuentas derivadas (PDA), lo que garantiza unicidad y seguridad.

🏗️ Arquitectura
Owner (Wallet)
    │
    └── Tienda (PDA)
            │
            ├── Producto A (PDA)
            ├── Producto B (PDA)
            └── Producto C (PDA)
            
🧱 Estructuras Principales


🏪 Tienda
Campo	Tipo	Descripción
owner	Pubkey	Wallet del propietario
nombre	String	Nombre de la tienda
ubicacion	String	Dirección física
hora_apertura	u8	Hora de apertura
hora_cierre	u8	Hora de cierre

💋 Producto
Campo	Tipo	Descripción
tienda	Pubkey	Tienda a la que pertenece
nombre	String	Nombre del producto
categoria	String	Ej: Labial, Base, Perfume
precio	u64	Precio del producto
stock	u16	Cantidad disponible

⚙️ Instrucciones del Programa
Instrucción	Descripción
crear_tienda(nombre, ubicacion, apertura, cierre)	Crea la tienda vinculada al propietario
agregar_producto(nombre, categoria, precio, stock)	Registra un nuevo producto
actualizar_precio(nombre, nuevo_precio)	Cambia el precio del producto
actualizar_stock(nombre, cantidad)	Modifica inventario
eliminar_producto(nombre)	Cierra la cuenta del producto
🔐 PDA (Program Derived Addresses)

Las cuentas se derivan con semillas:

Tienda:

["tienda", nombre_tienda, owner_pubkey]

Producto:

["producto", nombre_producto, owner_pubkey]

Esto garantiza que:

Cada propietario tiene su propia tienda única.

No pueden existir productos duplicados con el mismo nombre.

Solo el dueño puede modificar su tienda.

🚀 Cómo usar el proyecto en Solana Playground

1️⃣ Abrir Solana Playground
2️⃣ Hacer fork o pegar el contenido en src/lib.rs
3️⃣ Conectar wallet en Devnet
4️⃣ Presionar Build
5️⃣ Presionar Deploy
6️⃣ Ir a la sección Test

📌 Flujo de ejemplo
1. crear_tienda("BeautyStore", "Av. Juarez 123", 9, 20)

2. agregar_producto("Labial Dior", "Labial", 1500, 25)

3. actualizar_stock("Labial Dior", 40)

4. actualizar_precio("Labial Dior", 1700)

5. eliminar_producto("Labial Dior")
🛠️ Tecnologías

Solana — Blockchain de alta velocidad

Anchor — Framework para contratos en Solana

Rust — Lenguaje del programa

👩🏻‍💻 Autora

Proyecto desarrollado por Valeria Gallegos como parte de su práctica y aprendizaje en desarrollo Web3 sobre Solana.

](https://github.com/ValeriaGaEu/Proyecto-Cosmeticos/blob/main/proyecto/src/lib.rs)
