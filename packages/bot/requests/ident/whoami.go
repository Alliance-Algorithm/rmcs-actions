package ident

import (
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/requests"
	"github.com/google/uuid"
)

type WhoAmIRequestBody struct {
	UserName string `json:"username"`
	Mac      string `json:"mac"`
}

type WhoAmIResponse struct {
	RobotId   string    `json:"robot_id"`
	RobotUuid uuid.UUID `json:"robot_uuid"`
	RobotName string    `json:"robot_name"`
}

type WhoAmIRequest = requests.BaseRequest[WhoAmIRequestBody, WhoAmIResponse]

func NewWhoAmIRequest(body WhoAmIRequestBody) WhoAmIRequest {
	return WhoAmIRequest{
		EndpointPath: "/ident/whoami",
		HTTPMethod:   "POST",
		Body:         body,
	}
}
