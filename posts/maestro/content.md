This is a ***really*** scuffed wiki-esque post for my findings from reverse engineering 
"launcher.maestro.dll".
I don't really have a better way to host this currently, so I'm just setting this up as a blog post 
until I finally relent and create a Wiki system for this site.

***NOTE: ALL names were determined by me whilst reverse-engineering, only names that were public will be 
exactly what the Riot devs wrote!***

# MaestroClient

## Struct

| Name          | Type                          | Description                                 |
|---------------|-------------------------------|---------------------------------------------|
| messageAgent  | [MessageAgent](#messageagent) | Maestro message agent, handles all messages |
| socket        | SOCKET                        | Maestro socket                              |
| clientStarted | int                           | Is the client initialised?                  |

## Methods

| Function                                | Parameters                                                       | Returns | Description                                                                                                           |
|-----------------------------------------|------------------------------------------------------------------|---------|-----------------------------------------------------------------------------------------------------------------------|
| MaestroClient_Create                    | MaestroClient* [OUT]                                             | int     | Always returns one, MaestroClient_Start must be called after                                                          |
| MaestroClient_Delete                    | MaestroClient* [IN]                                              | void    | Closes socket and frees the client pointer                                                                            |
| MaestroClient_GetHeartbeatActive        | MaestroClient* [IN], int *isActive [OUT]                         | int     | Always returns one, isActive is modified to equal [messageAgent's](#messageagent) heartbeat_active field              |
| MaestroClient_SendMessage               | MaestroClient* [IN], int messageType, char *messageContents [IN] | void    | Internally calls [messageAgents's](#messageagent) MaestroMessageAgent_SendMessage                                     |
| MaestroClient_SetHeartbeatActive        | MaestroClient* [IN], int setActive                               | void    | Sets [messageAgent's](#messageagent) heartbeatActive to setActive                                                     |
| MaestroClient_SetMessageArrivedCallback | MaestroClient* [IN], void callbackOne, void callbackTwo          | void    | Sets callback one and two to their respective parameters                                                              |
| MaestroClient_Start                     | MaestroClient* [IN], u_short socketAddr                          | void    | Connects socket to the given socketAddr, starts [messageAgent](#messageagent), and sets all relevant started booleans |

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


| Function                           | Parameters         | Returns | Description                                                                                                                                |
|------------------------------------|--------------------|---------|--------------------------------------------------------------------------------------------------------------------------------------------|
| __fastcall Maestro_Receive_Message | MessageAgent* [IN] | int     | Calls Callback two with args: `Callback One, Message Type, Message Contents` on successful message receive. Returns 0 on error.            |
| Maestro_Client_Thread              | MessageAgent* [IN] | void    | Calls Maestro_Receive_Message until agentActive is 0 or an error has occurred. Sleeps 1ms every loop and sets heartbeatActive to 0 on exit |
| Maestro_Heartbeat_Thread           | MessageAgent* [IN] | void    | Calls MaestroMessageAgent_SendMessage with an ID of 4 every 25s.                                                                           |

