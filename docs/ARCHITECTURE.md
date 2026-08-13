# ARANDU Universal Architecture

## 1. Objetivo

O ARANDU CORE existe para impedir que o paradigma de navegação fique preso ao GNOME. A interface visual pode mudar; os três rios, as intenções e os perfis permanecem.

## 2. Camadas

```text
Input Sources
    ↓
Intent Translation
    ↓
ARANDU CORE
    ↓
Three Rivers State
    ↓
Platform Adapter + Visual Shell
```

## 3. Três rios

### MEMÓRIA
Lista limitada de aplicativos em ordem de recência. Capacidade padrão: 5.

### ABERTOS
Estado presente entregue pelo adaptador da plataforma.

### FREQUENTES
Ranking derivado de dados brutos de uso. A versão 0.1 mantém `launch_count`, `focus_count` e `active_seconds` e calcula um score simples e determinístico.

## 4. Confluência

A Confluência é um conceito lógico que une os três rios. Não é um aplicativo e não pertence a um sistema operacional. A camada visual decide como desenhá-la.

## 5. Intenções

O Core recebe intenções, não teclas, palavras ou gestos:

- `SelectRiver`
- `NextNode`
- `PreviousNode`
- `ActivateNode`
- `ReturnToConfluence`
- `OpenControlCenter`
- `Cancel`

## 6. Multimodalidade

Uma tecla, um botão do mouse, a palavra “MEMÓRIA”, um som médio ou um gesto de cabeça podem mapear para a mesma intenção.

## 7. Adaptadores

`PlatformAdapter` abstrai listagem, foco, ativação, lançamento e inventário de aplicativos. Implementações futuras:

- GNOME
- KDE
- Linux genérico
- Windows
- macOS

## 8. Privacidade

Entradas assistivas futuras devem ser módulos opcionais. Áudio e vídeo não fazem parte do Core e não devem ser persistidos por padrão.
