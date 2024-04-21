# Introdução

A partir da url de um website um específico, encurtadores de url são websites que realizam uma codificação de tal url. Tal url encurtada facilita o compartilhamento de url específicas. Acesso às urls encurtadas levam à url desejada através de redirecionamento.

# Requisitos 1


Title: Encurtamento de url arbitrária
Como usuário, 
Eu gostaria de encurtar uma url específica e ter acesso à url encurtada,
para poder usá-la e compartilhá-la.

Cenário 1: Encurtamento Google
When I shorten google.com
Then it should return a coded url that is not google.com

Cenário 2: Encurtamento Bing
When I shorten bing.com
Then it should return a coded url that is not bing.com


Título: Acesso de url encurtada
Como usuário, 
eu gostaria de poder ser redirecionado para um site original a partir da sua url encurtada,
para ter acesso ao que foi encurtado.

Cenário 1: Acesso url encurtada do Google
Given shortened google.com url
When I access it
Then it should redirect me to google.com

Cenário 2: Acesso url encurtada do Bing
Given shortened bing.com url
When I access it
Then it should redirect me to bing.com