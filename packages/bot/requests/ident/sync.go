package ident

import (
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/requests"
	"github.com/google/uuid"
)

type SyncRequestBody struct {
	RobotId string    `json:"robot_id"`
	Mac     string    `json:"mac"`
	Name    string    `json:"name"`
	Uuid    uuid.UUID `json:"uuid"`
}

type SyncResponse struct {
	Success bool `json:"success"`
}

type SyncRequest = requests.BaseRequest[SyncRequestBody, SyncResponse]

func NewSyncRequest(body SyncRequestBody) SyncRequest {
	return SyncRequest{
		EndpointPath: "/ident/sync",
		HTTPMethod:   "POST",
		Body:         body,
	}
}

type RetrieveRequestBody struct {
	Username   string `json:"username"`
	MacAddress string `json:"mac_address"`
}

type RetrieveResponse struct {
	RobotId string    `json:"robot_id"`
	Mac     string    `json:"mac"`
	Name    string    `json:"name"`
	Uuid    uuid.UUID `json:"uuid"`
}

type RetrieveRequest = requests.BaseRequest[RetrieveRequestBody, RetrieveResponse]

func NewRetrieveRequest(body RetrieveRequestBody) RetrieveRequest {
	return RetrieveRequest{
		EndpointPath: "/ident/retrieve",
		HTTPMethod:   "GET",
		Body:         body,
	}
}
