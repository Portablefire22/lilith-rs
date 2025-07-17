This is a ***really*** scuffed wiki-esque post for my findings from reverse engineering 
"launcher.maestro.dll".
I don't really have a better way to host this currently, so I'm just setting this up as a blog post 
until I finally relent and create a Wiki system for this site.

***NOTE: ALL names were determined by me whilst reverse-engineering, only names that were public will be 
exactly what the Riot devs wrote!***

# MaestroGameController

## Struct 

| Name           | Type                             | Description                                                       |
|----------------|----------------------------------|-------------------------------------------------------------------|
| maestroClient  | [MaestroClient](#maestroclient)* | Maestro client, seemingly handles interaction with the Launcher   |
| processNameOne | char*                            | Contains process name for MaestroGameController_KillProcessByName |
| processNameTwo | char*                            | Contains process name for MaestroGameController_KillProcessByName |

## Methods 

Many functions do not take MaestroGameController as a parameter and instead rely on a global created in 
MaestroGameController_Init


| Function                                               | Parameters                                           | Returns | Description                                                                                                                                              |
|--------------------------------------------------------|------------------------------------------------------|---------|----------------------------------------------------------------------------------------------------------------------------------------------------------|
| MaestroGameController_AreProcessesHung                 | int* isHung [OUT]                                    | void    | Sets isHung to 1 if processes are hung. (Need more info)                                                                                                 |
| MaestroGameController_GetHeartbeatActive               | int* isHeartbeatActive [OUT]                         | void    | Sets isHeartbeatActive equal to [messageAgent's](#messageAgent) heartbeatActive.                                                                         |
| MaestroGameController_Init                             | char* processNameOne [IN], char* processNameTwo [IN] | int     | Sets processName one and two respectively. Sets global GameController pointer to newly created GameController                                            |
| MaestroGameController_KillProcesses                    | void                                                 | void    | Kills processNameOne and Two, "rads_user_kernel.exe", "JojoDiff.exe", and "JojoPatch.exe". The last two seemingly being PandaMediaBooster related EXE's. |
| MaestroGameController_Remove                           | void                                                 | void    | Executes MaestroClient_Delete and frees the global pointer.                                                                                              |
| MaestroGameController_SendChatMessage                  | char* messageContent [IN]                            | void    | Calls MaestroMessageAgent_SendMessage with [messageType](#maestromessagetype) CHATMESSAGE_FROM_GAME.                                                     |
| MaestroGameController_SendGameAbandonedMessage         | void                                                 | void    | Calls MaestroMessageAgent_SendMessage with [messageType](#maestromessagetype) GAMECLIENT_ABANDONED.                                                      |
| MaestroGameController_SendGameConnectedToServerMessage | void                                                 | void    | Calls MaestroMessageAgent_SendMessage with [messageType](#maestromessagetype) GAMECLIENT_CONNECTED_TO_SERVER.                                            |
| MaestroGameController_SendGameLaunchedMessage          | void                                                 | void    | Calls MaestroMessageAgent_SendMessage with [messageType](#maestromessagetype) GAMECLIENT_LAUNCHED.                                                       |
| MaestroGameController_SendGameVersionMismatchMessage   | void                                                 | void    | Calls MaestroMessageAgent_SendMessage with [messageType](#maestromessagetype) GAMECLIENT_VERSION_MISMATCH.                                               |
| MaestroGameController_SendShutdownMessage              | void                                                 | void    | Calls MaestroMessageAgent_SendMessage with [messageType](#maestromessagetype) GAMECLIENT_STOPPED.                                                        |
| MaestroGameController_SetExitCallback                  |                                                      | void    |                                                                                                                                                          |
| MaestroGameController_SetHeartbeatActive               | int setActive                                        | void    | Sets [messageAgent's](#messageagent) heartbeatActive to setActive.                                                                                       |
| MaestroGameController_SetReceiveChatMessageCallback    |                                                      | void    |                                                                                                                                                          |
| MaestroGameController_Start                            | u_short socketAddr                                   | void    | Internall calls MaestroClient_Start with child maestroClient and socketAddr.                                                                             |

# MaestroClient

## Struct

| Name          | Type                          | Description                                 |
|---------------|-------------------------------|---------------------------------------------|
| messageAgent  | [MessageAgent](#messageagent) | Maestro message agent, handles all messages |
| socket        | SOCKET                        | Maestro socket                              |
| clientStarted | int                           | Is the client initialised?                  |

## Methods

| Function                                | Parameters                                                       | Returns | Description                                                                                                                                                                 |
|-----------------------------------------|------------------------------------------------------------------|---------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| MaestroClient_Create                    | MaestroClient* [OUT]                                             | int     | Always returns one, MaestroClient_Start must be called after                                                                                                                |
| MaestroClient_Delete                    | MaestroClient* [IN]                                              | void    | Closes socket and frees the client pointer                                                                                                                                  |
| MaestroClient_GetHeartbeatActive        | MaestroClient* [IN], int *isActive [OUT]                         | int     | Always returns one, isActive is modified to equal [messageAgent's](#messageagent) heartbeat_active field                                                                    |
| MaestroClient_SendMessage               | MaestroClient* [IN], int messageType, char *messageContents [IN] | void    | Internally calls [messageAgents's](#messageagent) MaestroMessageAgent_SendMessage                                                                                           |
| MaestroClient_SetHeartbeatActive        | MaestroClient* [IN], int setActive                               | void    | Sets [messageAgent's](#messageagent) heartbeatActive to setActive                                                                                                           |
| MaestroClient_SetMessageArrivedCallback | MaestroClient* [IN], void callbackOne, void callbackTwo          | void    | Sets callback one and two to their respective parameters                                                                                                                    |
| MaestroClient_Start                     | MaestroClient* [IN], u_short socketAddr                          | void    | Connects socket to the given socketAddr, starts [messageAgent](#messageagent), and sets all relevant started booleans                                                       |
| MaestroMessageAgent_Create              | MaestroClient* [IN/OUT]                                          | int     | Creates a blank [messageAgent](#messageagent) and assigns it to MaestroClient's messageAgent field                                                                          |
| MaestroMessageAgent_Delete              | MaestroClient* [IN/OUT]                                          | void    | Sets [messageAgent](#messageagent)'s agentActive field to 0 and waits for all associated threads to shutdown. Closes relevant thread handles and frees messageAgent pointer |

# MessageAgent 

## Struct

| Name                     | Type               | Description                                             |
|--------------------------|--------------------|---------------------------------------------------------|
| socket                   | SOCKET             | Socket for sending & receiving maestro messages         |
| messageAgentThread       | HANDLE             | Thread handle for receiving maestro messages.           |
| heartbeatThread          | HANDLE             | Thread handle for sending heartbeats                    |
| agentActive?             | int                | Boolean for if message agent is active/initialised      |
| heartbeatActive?         | int                | Boolean for if heartbeats should be sent                |
| lastHeartbeatTimeSeconds | DWORD              | When the last heartbeat was sent as System time seconds |
| callbackOne              | Not yet determined | Not yet determined                                      |
| callbackTwo              | Not yet determined | Not yet determined                                      |

## Methods


| Function                                      | Parameters                                                       | Returns | Description                                                                                                                                                                         |
|-----------------------------------------------|------------------------------------------------------------------|---------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| __fastcall Maestro_Receive_Message            | MessageAgent* [IN]                                               | int     | Calls Callback two with args: `Callback One, Message Type, Message Contents` on successful message receive. Returns 0 on error.                                                     |
| Maestro_Client_Thread                         | MessageAgent* [IN]                                               | void    | Calls Maestro_Receive_Message until agentActive is 0 or an error has occurred. Sleeps 1ms every loop and sets heartbeatActive to 0 on exit                                          |
| Maestro_Heartbeat_Thread                      | MessageAgent* [IN]                                               | void    | Calls MaestroMessageAgent_SendMessage with an ID of 4 every 25s.                                                                                                                    |
| MaestroMessageAgent_GetHeartbeatActive        | MessageAgent* [IN], int* isHeartbeatActive [OUT]                 | int     | Sets isHeartbeatActive value equal to heartbeatActive. Always returns one.                                                                                                          |
| MaestroMessageAgent_GetIsRunning              | MessageAgent* [IN], int* isActive [OUT]                          | int     | Sets isActive value equal to agentActive. Always returns one.                                                                                                                       |
| MaestroMessageAgent_SendMessage               | MessageAgent* [IN], int messageType, char* messageContents [IN]  | void    | Sends message to messageAgent's socket. Will seemingly fail if given a non-null terminated character array. Logs message if shouldLogHeartbeat == 1 or if messageType is not 4 or 5 |
| MaestroMessageAgent_GetHeartbeatActive        | MessageAgent* [IN], int* isHeartbeatActive [OUT]                 | int     | Sets isHeartbeatActive value equal to heartbeatActive. Always returns one.                                                                                                          |
| MaestroMessageAgent_SetHeartbeatActive        | MessageAgent* [IN/OUT], int setActive                            | int     | Sets heartbeatActive to setActive. Always returns one.                                                                                                                              |
| MaestroMessageAgent_SetMessageArrivedCallback | MessageAgent* [IN/OUT], UNKNOWN callbackOne, UNKNOWN callbackTwo | int     | Sets callback one and two respectively. Always returns one.                                                                                                                         |
| MaestroMessageAgent_Start                     | MessageAgent* [IN/OUT], SOCKET socket                            | int     | Sets socket to given socket and agentActive to one. Checks agentActive before starting. Returns 0 on error.                                                                         |

## MaestroMessageType

| MessageType                                              | Value   |
|----------------------------------------------------------|---------|
| MAESTROMESSAGETYPE_GAMECLIENT_CREATE                     | 0       |
| MAESTROMESSAGETYPE_GAMECLIENT_STOPPED                    | 1       |
| MAESTROMESSAGETYPE_GAMECLIENT_CRASHED                    | 2       |
| MAESTROMESSAGETYPE_CLOSE                                 | 3       |
| MAESTROMESSAGETYPE_HEARTBEAT                             | 4       |
| MAESTROMESSAGETYPE_REPLY                                 | 5       |
| MAESTROMESSAGETYPE_LAUNCHERCLIENT                        | 6       |
| MAESTROMESSAGETYPE_GAMECLIENT_ABANDONED                  | 7       |
| MAESTROMESSAGETYPE_GAMECLIENT_LAUNCHED                   | 8       |
| MAESTROMESSAGETYPE_GAMECLIENT_VERSION_MISMATCH           | 9       |
| MAESTROMESSAGETYPE_GAMECLIENT_CONNECTED_TO_SERVER        | 10      |
| MAESTROMESSAGETYPE_CHATMESSAGE_TO_GAME                   | 11      |
| MAESTROMESSAGETYPE_CHATMESSAGE_FROM_GAME                 | 12      |
| MAESTROMESSAGETYPE_GAMECLIENT_CREATE_VERSION             | 13      |
| MAESTROMESSAGETYPE_GAMECLIENT_INSTALL_VERSION            | 14      |
| MAESTROMESSAGETYPE_GAMECLIENT_CANCEL_INSTALL             | 15      |
| MAESTROMESSAGETYPE_GAMECLIENT_INSTALL_PROGRESS           | 16      |
| MAESTROMESSAGETYPE_GAMECLIENT_INSTALL_PREVIEW            | 17      |
| MAESTROMESSAGETYPE_GAMECLIENT_CANCEL_PREVIEW             | 18      |
| MAESTROMESSAGETYPE_GAMECLIENT_PREVIEW_PROGRESS           | 19      |
| MAESTROMESSAGETYPE_PLAY_REPLAY                           | 20      |
| MAESTROMESSAGETYPE_GAMECLIENT_UNINSTALL_VERSION          | 21      |
| MAESTROMESSAGETYPE_GAMECLIENT_CANCEL_UNINSTALL           | 22      |
| MAESTROMESSAGETYPE_GAMECLIENT_UNINSTALL_PROGRESS         | 23      |
| MAESTROMESSAGETYPE_GAMECLIENT_UNINSTALL_PREVIEW          | 24      |
| MAESTROMESSAGETYPE_GAMECLIENT_CANCEL_UNINSTALL_PREVIEW   | 25      |
| MAESTROMESSAGETYPE_GAMECLIENT_PREVIEW_UNINSTALL_PROGRESS | 26      |
| MAESTROMESSAGETYPE_GAMECLIENT_ENUMERATE_VERSIONS         | 27      |
| MAESTROMESSAGETYPE_GAMECLIENT_CREATE_CLIENT_AND_PRELOAD  | 28      |
| MAESTROMESSAGETYPE_GAMECLIENT_START_PRELOADED_GAME       | 29      |
| MAESTROMESSAGETYPE_INVALID                               | default |

# MaestroServer 

## Struct 
| Name   | Type    | Description   |
|--------|---------|---------------|
| socket | Socket  | Server socket |
## Methods